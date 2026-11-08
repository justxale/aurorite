use crate::extractors::AuthorizedClient;
use crate::requests::PostSessionMessage;
use crate::responses::{
    AuroriteErrorResponse, FailableResponse, MessageInfo, SessionClientInfo, SessionInfo,
};
use crate::session::WebsocketMessage;
use crate::state::AuroriteState;
use crate::traits::IntoJson;
use aurorite_util::uuid::EncodedUuid;
use axum::extract::{Json, Path, State, WebSocketUpgrade};
use axum::http::StatusCode;
use axum::response::Response;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

#[utoipa::path(
    get,
    path = "/",
    params(("session_id", description = "ID of session")),
    tag = "sessions",
    responses((status = OK, description = "Session details"), (status = NOT_FOUND, body = AuroriteErrorResponse))
)]
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

#[utoipa::path(
    get,
    path = "/ws",
    params(("session_id", description = "ID of session")),
    tag = "sessions",
    responses((status = SWITCHING_PROTOCOLS, description = "WebSocket connection established"), (status = NOT_FOUND, body = AuroriteErrorResponse))
)]
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

#[utoipa::path(
    post,
    path = "/chat",
    params(("session_id", description = "ID of session")),
    tag = "sessions",
    security(("password" = [])),
    responses((status = ACCEPTED, description = "Message accepted"), (status = NOT_FOUND, body = AuroriteErrorResponse))
)]
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
            .broadcast(WebsocketMessage::OnMessage {
                content: body.content,
                created_at: now,
                client: info,
            })
            .await
    });
    Ok((StatusCode::ACCEPTED, response.json()))
}

pub fn build_root_routes() -> OpenApiRouter<AuroriteState> {
    OpenApiRouter::new()
        .routes(routes!(get_session, post_message))
        .routes(routes!(handle_ws))
}
