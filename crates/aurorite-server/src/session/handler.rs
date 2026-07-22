use crate::session::WebsocketMessage;
use aurorite_agsp::loader::load_text_file;
use aurorite_dataflow::database::{Campaign, CampaignCharacter, Db, Scene, Script};
use aurorite_dataflow::dto::SceneDto;
use aurorite_runtime::{AuroriteRuntime, CachedScript, RuntimeCtx, RuntimeEvent, ScriptSchema};
use aurorite_util::jwt::decode_key;
use axum::extract::ws::{CloseFrame, Message, Utf8Bytes, WebSocket, close_code};
use dashmap::DashMap;
use dashmap::mapref::one::Ref;
use futures_util::stream::SplitSink;
use futures_util::{SinkExt, StreamExt};
use jiff::Timestamp;
use parking_lot::Mutex;
use std::sync::Arc;
use tokio::sync::mpsc::{Receiver, Sender, channel};
use tokio::task::JoinSet;
use uuid::Uuid;

const BUFFER_SIZE: usize = 128;

fn handle_event(set: &mut JoinSet<Result<(), SendEvent>>, event: SendEvent) {
    set.spawn(async move {
        event.sender.send(event.msg).await.map_err(|err| SendEvent {
            id: event.id,
            client_id: event.client_id,
            msg: err.0,
            sender: event.sender,
        })
    });
}

fn broadcast_owned(
    sockets: &Arc<DashMap<Uuid, DashMap<Uuid, Sender<WebsocketMessage>>>>,
    msg: WebsocketMessage
) {
    let mut set = JoinSet::new();

    for client in sockets.iter() {
        for conn in client.value().iter() {
            let event = SendEvent {
                id: *conn.key(),
                client_id: *client.key(),
                msg: msg.clone(),
                sender: conn.value().clone(),
            };
            handle_event(&mut set, event);
        }
    }
}

async fn handle_set_owned(mut set: JoinSet<Result<(), SendEvent>>) -> Result<(), SendEvent> {
    let mut retry_set = JoinSet::new();
    while let Some(res) = set.join_next().await {
        if let Ok(res) = res
            && let Err(event) = res
        {
            tracing::warn!("trying to resend message");
            handle_event(&mut retry_set, event);
        }
    }
    while let Some(res) = retry_set.join_next().await {
        if res.is_err() {
            tracing::error!("unspecified error during execution");
        }
        if let Ok(res) = res
            && let Err(event) = res
        {
            tracing::error!("failed to send message after retry");
            return Err(event);
        }
    }
    Ok(())
}

struct SendEvent {
    pub id: Uuid,
    pub client_id: Uuid,
    pub msg: WebsocketMessage,
    pub sender: Sender<WebsocketMessage>,
}

#[derive(Debug)]
pub struct SessionClient {
    pub id: Uuid,
    pub name: String,
}

pub struct Session {
    campaign_id: Uuid,
    db: Db,
    clients: DashMap<Uuid, SessionClient>,
    guests: DashMap<Uuid, SessionClient>,
    sockets: Arc<DashMap<Uuid, DashMap<Uuid, Sender<WebsocketMessage>>>>,
    ctx: Arc<Mutex<RuntimeCtx>>,
    rt: AuroriteRuntime,

    pub started_at: Timestamp,
}

impl Session {
    pub fn new(campaign_id: Uuid, db: Db) -> Self {
        let (sender, reader) = channel::<RuntimeEvent>(BUFFER_SIZE);
        let ctx = Arc::new(Mutex::new(RuntimeCtx::new(campaign_id, sender)));
        let session = Self {
            campaign_id,
            db,
            clients: DashMap::new(),
            guests: DashMap::new(),
            sockets: Arc::new(DashMap::new()),
            ctx: ctx.clone(),
            rt: AuroriteRuntime::new(ctx),
            started_at: Timestamp::now(),
        };
        tokio::spawn(Self::handle_event_stream(session.sockets.clone(), reader));
        session
    }

    #[inline]
    pub fn ctx(&self) -> &Arc<Mutex<RuntimeCtx>> {
        &self.ctx
    }

    #[inline]
    pub fn clients(&self) -> &DashMap<Uuid, SessionClient> {
        &self.clients
    }

    #[inline]
    pub fn guests(&self) -> &DashMap<Uuid, SessionClient> {
        &self.guests
    }

    pub async fn attach(&self, mut socket: WebSocket) {
        let err = Message::Close(Some(CloseFrame {
            code: close_code::POLICY,
            reason: Utf8Bytes::from_static("unauthorized"),
        }));
        let msg = if let Some(msg) = socket.recv().await {
            msg
        } else {
            return;
        };
        if let Ok(ref msg) = msg
            && let Ok(msg) = WebsocketMessage::try_from(msg)
            && let WebsocketMessage::Auth { token } = msg
        {
            let payload = if let Ok(payload) = decode_key(&token) {
                payload
            } else {
                let _ = socket.send(err).await;
                return;
            };
            tracing::info!(
                "websocket for {} {} attached",
                if payload.is_guest.unwrap_or(false) {
                    "guest"
                } else {
                    "client"
                },
                payload.id()
            );
            let (sender, reader) = channel::<WebsocketMessage>(BUFFER_SIZE);
            self.sockets
                .entry(payload.id())
                .or_default()
                .insert(Uuid::now_v7(), sender);
            let (ws_sender, _) = socket.split();
            tokio::spawn(Self::handle_message_stream(ws_sender, reader));
        } else {
            tracing::info!("unautorized websocket rejected");
            let _ = socket.send(err).await;
        }
    }

    pub async fn broadcast(&self, msg: WebsocketMessage) {
        broadcast_owned(&self.sockets, msg)
    }

    pub async fn send_to(&self, client_id: Uuid, msg: WebsocketMessage) {
        let mut set = JoinSet::new();

        let connections = if let Some(set) = self.sockets.get(&client_id) {
            set
        } else {
            return;
        };
        for conn in connections.iter() {
            let event = SendEvent {
                id: *conn.key(),
                client_id,
                msg: msg.clone(),
                sender: conn.value().clone(),
            };
            handle_event(&mut set, event);
        }
        self.handle_set(set).await;
    }

    async fn handle_message_stream(
        mut sink: SplitSink<WebSocket, Message>,
        mut stream: Receiver<WebsocketMessage>,
    ) {
        while let Some(event) = stream.recv().await {
            let _ = sink
                .send(Message::Text(Utf8Bytes::from(
                    serde_json::to_string(&event).unwrap(),
                )))
                .await;
        }
    }

    async fn handle_event_stream(
        sockets: Arc<DashMap<Uuid, DashMap<Uuid, Sender<WebsocketMessage>>>>,
        mut stream: Receiver<RuntimeEvent>
    ) {
        while let Some(event) = stream.recv().await {
            let msg = WebsocketMessage::from(event);
            broadcast_owned(&sockets, msg);
        }
    }

    async fn handle_set(&self, set: JoinSet<Result<(), SendEvent>>) {
        match handle_set_owned(set).await {
            Ok(_) => (),
            Err(SendEvent { id, client_id, msg: _, sender: _ }) => {
                if let Some(set) = self.sockets.get(&client_id) {
                    set.remove(&id);
                }
            }
        }
    }

    pub async fn load_campaign(&mut self) -> Result<(), &'static str> {
        let record = Campaign::filter_by_id(self.campaign_id)
            .include(Campaign::fields().scene())
            .include(Campaign::fields().scene().preloads())
            .include(Campaign::fields().scene().preloads().character())
            .include(Campaign::fields().scene().asset())
            .include(Campaign::fields().characters())
            .get(&mut self.db)
            .await
            .map_err(|_| "failed to load campaign")?;

        let mut lock = self.ctx.lock();
        if let Some(dto) = record
            .scene
            .get()
            .as_ref()
            .and_then(|s| SceneDto::try_from(s).ok())
        {
            lock.switch_scene(dto);
        } else {
            lock.remove_scene();
        }
        Ok(())
    }

    pub async fn load_scene(&mut self, scene_id: Uuid) -> Result<(), &'static str> {
        match Scene::filter_by_id(scene_id)
            .include(Scene::fields().preloads())
            .include(Scene::fields().asset())
            .get(&mut self.db)
            .await
        {
            Err(_) => Err("scene not found"),
            Ok(s) => {
                let mut lock = self.ctx.lock();
                lock.switch_scene(SceneDto::try_from(&s)?);
                Ok(())
            }
        }
    }

    pub async fn load_spell(
        &mut self,
        character_id: Uuid,
        spell_id: Uuid,
    ) -> Result<(), &'static str> {
        let (t, asset) = {
            let ctx = self.ctx.lock();
            let s = ctx
                .character(character_id)
                .and_then(|c| c.spell(spell_id))
                .ok_or("failed to load spell")?;
            match s.script {
                Script::Python => unimplemented!("python is not supported yet"),
                Script::Vismut => (Script::Vismut, s.script_asset.clone()),
            }
        };
        let script = match t {
            Script::Python => unimplemented!(),
            Script::Vismut => {
                let content = match load_text_file(asset).await {
                    Ok(c) => c,
                    Err(_) => return Err("script loading failed"),
                };
                let schema =
                    serde_json::from_str::<ScriptSchema>(&content).map_err(|_| "invalid json")?;
                self.rt
                    .parse(&schema)
                    .map_err(|_| "registry error occured")?
            }
        };
        self.ctx.lock().character_mut(character_id).and_then(|c| {
            c.spell_mut(spell_id)
                .map(|s| s.cached_script = CachedScript::Vismut(script))
        });

        Ok(())
    }

    async fn save_state(&self, db: &mut Db) -> Result<(), &'static str> {
        let map = self.ctx.lock().characters_current_hits();
        let mut tx = db.transaction().await.map_err(|_| "db failure")?;
        for (id, hits) in map {
            let _ = CampaignCharacter::update_by_character_id_and_campaign_id(id, self.campaign_id)
                .current_hits(hits)
                .exec(&mut tx)
                .await;
        }
        tx.commit()
            .await
            .map_err(|_| "transaction failed, data will not be saved")?;
        Ok(())
    }

    async fn cleanup(self, db: &mut Db) {
        let (_, _) = tokio::join!(
            self.broadcast(WebsocketMessage::Shutdown {
                reason: Some("disconnecting".to_string())
            }),
            self.save_state(db)
        );
    }
}

pub struct SessionManager {
    sessions: DashMap<Uuid, Session>,
    db: Db,
}

impl SessionManager {
    pub fn new(db: Db) -> Self {
        Self {
            db,
            sessions: DashMap::new(),
        }
    }

    #[inline]
    pub fn sessions(&self) -> &DashMap<Uuid, Session> {
        &self.sessions
    }

    #[inline]
    pub fn session(&self, session_id: Uuid) -> Option<Ref<'_, Uuid, Session>> {
        self.sessions.get(&session_id)
    }

    #[inline]
    pub fn attach(&self, campaign_id: Uuid) {
        self.sessions
            .insert(campaign_id, Session::new(campaign_id, self.db.clone()));
    }

    #[inline]
    pub fn detach(&self, campaign_id: Uuid) {
        self.sessions.remove(&campaign_id);
    }

    pub async fn cleanup(mut self) {
        for (_id, session) in self.sessions.into_iter() {
            session.cleanup(&mut self.db).await;
        }
    }
}
