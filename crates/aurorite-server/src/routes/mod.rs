mod agsp;
mod backgrounds;
mod campaigns;
mod characters;
mod classes;
mod client;
mod races;

use crate::state::AuroriteState;
use axum::extract::{MatchedPath, Request};
use axum::response::Response;
use axum::routing::any;
use axum::{Router, http};
use http::StatusCode;
use std::time::Duration;
use tower::ServiceBuilder;
use tower_http::services::ServeDir;
use tower_http::timeout::TimeoutLayer;
use tower_http::trace::TraceLayer;
use tracing::Span;

pub fn build_routes() -> Router<AuroriteState> {
    Router::new()
        .nest("/characters", characters::build_characters_routes())
        .nest("/client", client::build_client_routes())
        .nest("/backgrounds", backgrounds::build_backgrounds_routes())
        .nest("/classes", classes::build_classes_routes())
        .nest("/campaigns", campaigns::build_campaign_routes())
        .nest("/agsp", agsp::build_agsp_routes())
        .route("/healthcheck", any(async || StatusCode::NO_CONTENT))
        .route_service("/", ServeDir::new("static"))
        .layer(
            ServiceBuilder::new()
                .layer(
                    TraceLayer::new_for_http()
                        .make_span_with(|request: &Request<_>| {
                            let matched_path = request
                                .extensions()
                                .get::<MatchedPath>()
                                .map(MatchedPath::as_str);
                            let mut request_id = ['0'; 8];
                            for c in &mut request_id {
                                *c = fastrand::alphanumeric();
                            }
                            tracing::info_span!(
                                "request",
                                method = ?request.method(),
                                matched_path,
                                request_id = request_id.into_iter().collect::<String>()
                            )
                        })
                        .on_request(|_request: &Request<_>, _span: &Span| {})
                        .on_response(|response: &Response, latency: Duration, _span: &Span| {
                            tracing::info!(latency = ?latency, status = ?response.status());
                        }),
                )
                .layer(TimeoutLayer::with_status_code(
                    StatusCode::REQUEST_TIMEOUT,
                    Duration::from_secs(5),
                )),
        )
}
