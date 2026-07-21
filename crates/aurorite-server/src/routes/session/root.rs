use crate::extractors::AuthorizedClient;
use crate::requests::PostSessionMessage;
use crate::responses::{
    AuroriteErrorResponse, FailableResponse, MessageInfo, SessionClientInfo, SessionInfo,
};
use crate::session::WebsocketMessage;
use crate::state::AuroriteState;
use crate::traits::IntoJson;
use aurorite_util::uuid::EncodedUuid;
use axum::Router;
use axum::extract::{Json, Path, State, WebSocketUpgrade};
use axum::http::StatusCode;
use axum::response::Response;
use axum::routing::{any, get, post};

async fn get_session(
    Path(EncodedUuid(id)): Path<EncodedUuid>,
    State(state): State<AuroriteState>,
) -> FailableResponse<SessionInfo> {
    let session = state.manager.sessions().get(&id);
    if session.is_none() {
        return Err((
            StatusCode::NOT_FOUND,
            AuroriteErrorResponse::new("root not found").json(),
        ));
    };
    Ok((
        StatusCode::OK,
        SessionInfo::from(session.unwrap().value()).json(),
    ))
}

async fn handle_ws(
    Path(EncodedUuid(id)): Path<EncodedUuid>,
    State(state): State<AuroriteState>,
    ws: WebSocketUpgrade,
) -> Result<Response, (StatusCode, Json<AuroriteErrorResponse>)> {
    if state.manager.sessions().get(&id).is_none() {
        return Err((
            StatusCode::NOT_FOUND,
            AuroriteErrorResponse::new("root not found").json(),
        ));
    }
    Ok(ws.on_upgrade(async move |socket| {
        state
            .manager
            .sessions()
            .get(&id)
            .unwrap()
            .value()
            .attach(socket)
            .await
    }))
}

async fn post_message(
    Path(EncodedUuid(id)): Path<EncodedUuid>,
    State(state): State<AuroriteState>,
    AuthorizedClient(client): AuthorizedClient,
    Json(body): Json<PostSessionMessage>,
) -> FailableResponse<MessageInfo> {
    if state.manager.session(id).is_none() {
        return Err((
            StatusCode::NOT_FOUND,
            AuroriteErrorResponse::new("root not found").json(),
        ));
    }
    let now = jiff::Timestamp::now();
    let info = SessionClientInfo {
        id: client.id,
        name: client.display_name.unwrap_or(client.username),
        is_guest: false,
    };
    let response = MessageInfo {
        created_at: now,
        client: info.clone(),
        content: body.content.clone(),
    };
    tokio::spawn(async move {
        state
            .manager
            .session(id)
            .unwrap()
            .value()
            .broadcast(WebsocketMessage::Chat {
                content: body.content,
                created_at: now,
                client: info,
            })
            .await
    });
    Ok((StatusCode::ACCEPTED, response.json()))
}

pub fn build_root_routes() -> Router<AuroriteState> {
    let id_router = Router::new()
        .route("/", get(get_session))
        .route("/ws", any(handle_ws))
        .route("/chat", post(post_message));
    Router::new().nest("/{session_id}", id_router)
}
