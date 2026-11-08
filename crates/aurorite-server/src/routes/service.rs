use crate::responses::{AuroriteErrorResponse, ServiceInfo};
use crate::state::{AuroriteState, SERVICE_EVENT_SENDER};
use crate::traits::IntoJson;
use axum::extract::{Query, Request, State};
use axum::http::StatusCode;
use axum::response::sse::{Event, KeepAlive};
use axum::response::{IntoResponse, Response, Sse};
use axum::http::Version;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;
use serde::Deserialize;
use std::time::Duration;
use tokio_stream::StreamExt;
use tokio_stream::wrappers::BroadcastStream;

#[derive(Deserialize)]
struct Q {
    stream: Option<bool>,
}

#[utoipa::path(
    get,
    path = "/",
    params(("stream" = Option<bool>, Query, description = "Stream service events over HTTP/2 or HTTP/3")),
    tag = "service",
    responses((status = OK, description = "Service information or event stream"), (status = HTTP_VERSION_NOT_SUPPORTED, description = "Streaming requires HTTP/2 or HTTP/3"))
)]
async fn get_service(State(state): State<AuroriteState>, Query(query): Query<Q>, request: Request) -> Response {
    match (query, request.version()) {
        (Q { stream: Some(true), }, Version::HTTP_2 | Version::HTTP_3, ) => {
            let stream = BroadcastStream::new(SERVICE_EVENT_SENDER.subscribe())
                .map(|ev| {
                    ev.map(|ev| Event::default().event(ev.event_name()).json_data(ev))
                        .map_err(axum::Error::new)
                        .flatten()
                }).map_while(move |v| if state.cancellation_token.is_cancelled() { None } else { Some(v) });
            Sse::new(stream)
                .keep_alive(KeepAlive::new().interval(Duration::from_secs(2)).text("keep-alive"))
                .into_response()
        }
        (Q { stream: Some(true), }, _, ) => (
            StatusCode::HTTP_VERSION_NOT_SUPPORTED,
            AuroriteErrorResponse::new("service stream is not available over HTTP/1.1").json()
        ).into_response(),
        _ => (StatusCode::OK, ServiceInfo::default().json()).into_response(),
    }
}

pub fn build_service_routes() -> OpenApiRouter<AuroriteState> {
    OpenApiRouter::new().routes(routes!(get_service))
}
