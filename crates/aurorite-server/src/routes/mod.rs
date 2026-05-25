mod characters;
mod client;
mod agsp;
mod campaigns;
mod classes;
mod races;
mod backgrounds;

use crate::state::AuroriteState;
use axum::extract::{Request, MatchedPath};
use axum::response::Response;
use axum::{Router, http};
use http::StatusCode;
use std::time::Duration;
use axum::routing::any;
use tower::ServiceBuilder;
use tower_http::services::ServeDir;
use tower_http::timeout::TimeoutLayer;
use tower_http::trace::TraceLayer;
use tracing::Span;

pub fn build_routes() -> Router<AuroriteState> {
    Router::new()
        // .nest("/characters", characters::build_characters_routes())
        .nest("/client", client::build_client_routes())
        .nest("/backgrounds", backgrounds::build_backgrounds_routes())
        .nest("/campaigns", campaigns::build_campaign_routes())
        .nest("/agsp", agsp::build_agsp_routes())
        .route("/healthcheck", any(async || StatusCode::NO_CONTENT))
        .route_service("/", ServeDir::new("static"))
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http()
                    .make_span_with(|request: &Request<_>| {
                        let matched_path = request
                            .extensions()
                            .get::<MatchedPath>()
                            .map(MatchedPath::as_str);

                        tracing::info_span!(
                            "request",
                            method = ?request.method(),
                            matched_path,
                        )
                    })
                    .on_request(|_request: &Request<_>, _span: &Span| {
                        // tracing::debug!("started processing request")
                    })
                    .on_response(|response: &Response, latency: Duration, _span: &Span| {
                        tracing::info!(latency = ?latency, status = ?response.status());
                    }))
                .layer(TimeoutLayer::with_status_code(StatusCode::REQUEST_TIMEOUT, Duration::from_secs(5)))
        )
}
