use futures_util::{StreamExt, SinkExt};
use axum::extract::ws::{close_code, CloseFrame, Message, Utf8Bytes, WebSocket};
use dashmap::DashMap;
use dashmap::mapref::one::Ref;
use futures_util::stream::SplitSink;
use jiff::Timestamp;
use tokio::sync::mpsc::{channel, Receiver, Sender};
use tokio::task::JoinSet;
use uuid::Uuid;
use aurorite_dataflow::database::{CampaignCharacter, Db};
use aurorite_util::jwt::decode_key;
use crate::session::character::Character;
use crate::session::scene::Scene;
use crate::session::WebsocketMessage;

struct SendEvent {
    pub id: Uuid,
    pub client_id: Uuid,
    pub msg: WebsocketMessage,
    pub sender: Sender<WebsocketMessage>,
}

#[derive(Debug)]
pub struct SessionClient {
    pub id: Uuid,
    pub name: String
}

#[derive(Debug)]
pub struct Session {
    campaign_id: Uuid,
    clients: DashMap<Uuid, SessionClient>,
    guests: DashMap<Uuid, SessionClient>,
    sockets: DashMap<Uuid, DashMap<Uuid, Sender<WebsocketMessage>>>,

    scene: Option<Scene>,
    characters: DashMap<Uuid, Character>,

    pub started_at: Timestamp,
}

impl Session {
    pub fn new(campaign_id: Uuid) -> Self {
        Self {
            campaign_id,
            clients: DashMap::new(),
            guests: DashMap::new(),
            sockets: DashMap::new(),
            scene: None,
            characters: DashMap::new(),
            started_at: Timestamp::now(),
        }
    }

    pub fn clients(&self) -> &DashMap<Uuid, SessionClient> {
        &self.clients
    }

    pub fn guests(&self) -> &DashMap<Uuid, SessionClient> {
        &self.guests
    }

    pub async fn attach(&self, mut socket: WebSocket) {
        let err = Message::Close(Some(CloseFrame { code: close_code::POLICY, reason: Utf8Bytes::from_static("unauthorized")}));
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
            tracing::info!("websocket for {} {} attached", if payload.is_guest.unwrap_or(false) { "guest" } else { "client" }, payload.id());
            let (sender, reader) = channel::<WebsocketMessage>(32);
            self.sockets
                .entry(payload.id())
                .or_default()
                .insert(Uuid::now_v7(), sender);
            let (ws_sender, _) = socket.split();
            tokio::spawn(Self::handle_sender(ws_sender, reader));
        } else {
            tracing::info!("unautorized websocket rejected");
            let _ = socket.send(err).await;
        }
    }

    pub async fn broadcast(&self, msg: WebsocketMessage) {
        let mut set = JoinSet::new();

        for client in self.sockets.iter() {
            for conn in client.value().iter() {
                let event = SendEvent {
                    id: *conn.key(),
                    client_id: *client.key(),
                    msg: msg.clone(),
                    sender: conn.value().clone()
                };
                Self::handle_event(&mut set, event);
            }
        }
        self.handle_set(set).await;
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
                sender: conn.value().clone()
            };
            Self::handle_event(&mut set, event);
        }
        self.handle_set(set).await;
    }

    async fn handle_sender(mut sink: SplitSink<WebSocket, Message>, mut stream: Receiver<WebsocketMessage>) {
        while let Some(event) = stream.recv().await {
            let _ = sink.send(Message::Text(Utf8Bytes::from(serde_json::to_string(&event).unwrap()))).await;
        }
    }

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

    async fn handle_set(&self, mut set: JoinSet<Result<(), SendEvent>>) {
        let mut retry_set = JoinSet::new();
        while let Some(res) = set.join_next().await {
            if let Ok(res) = res && let Err(event) = res {
                tracing::warn!("trying to resend message");
                Self::handle_event(&mut retry_set, event);
            }
        }
        while let Some(res) = retry_set.join_next().await {
            if res.is_err() {
                tracing::error!("unspecified error during execution");
            }
            if let Ok(res) = res && let Err(event) = res {
                tracing::error!("failed to send message after retry");
                if let Some(set) = self.sockets.get(&event.client_id) {
                    set.remove(&event.id);
                }
            }
        }
    }

    async fn save_state(&self, db: &mut Db) -> Result<(), &'static str> {
        let mut tx = db.transaction().await.map_err(|_| "db failure")?;
        for c in self.characters.iter() {
            let _ = CampaignCharacter::update_by_character_id_and_campaign_id(c.id, self.campaign_id)
                .current_hits(c.current_hits)
                .exec(&mut tx).await;
        }
        tx.commit().await.map_err(|_| "transaction failed, data will not be saved")?;
        Ok(())
    }

    async fn cleanup(self, mut db: Db) -> Db {
        let (_, _) = tokio::join!(
            self.broadcast(WebsocketMessage::Shutdown { reason: Some("disconnecting".to_string()) }),
            self.save_state(&mut db)
        );
        db
    }
}

#[derive(Debug)]
pub struct SessionManager {
    sessions: DashMap<Uuid, Session>,
}

impl SessionManager {
    pub fn new() -> Self {
        Self {
            sessions: DashMap::new(),
        }
    }

    #[inline]
    pub fn sessions(&self) -> &DashMap<Uuid, Session> {
        &self.sessions
    }

    #[inline]
    pub fn session(&self, session_id: Uuid) ->  Option<Ref<'_, Uuid, Session>>
    {
        self.sessions.get(&session_id)
    }

    #[inline]
    pub fn attach(&self, campaign_id: Uuid) {
        self.sessions.insert(campaign_id, Session::new(campaign_id));
    }

    #[inline]
    pub fn detach(&self, campaign_id: Uuid) {
        self.sessions.remove(&campaign_id);
    }

    pub async fn cleanup(self, db: Db) {
        let mut db = db;
        for s in self.sessions.iter() {
            let session = if let Some(s) = self.sessions.remove(s.key()) {
                s.1
            } else {
                unreachable!("cleanup failure");
            };
            db = session.cleanup(db).await;
        }
    }
}