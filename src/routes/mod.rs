mod characters;
mod client;

use crate::state::AuroriteState;
use axum::extract::Request;
use axum::response::Response;
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
        // .nest("/characters", characters::build_characters_routes())
        .nest("/client", client::build_client_routes())
        .route_service("/", ServeDir::new("static"))
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http()
                    .make_span_with(|_: &Request| {
                        tracing::info_span!(
                            "api",
                            path = tracing::field::Empty,
                            err = tracing::field::Empty,
                        )
                    })
                    .on_request(|req: &Request, span: &Span| {
                        span.record("path", tracing::field::display(&req.uri().path()));
                    })
                    .on_response(|response: &Response, duration: Duration, span: &Span| {
                        if !response.status().is_server_error() {
                            tracing::info!(parent: span, duration = ?duration.as_millis(), status_code = ?response.status());
                        }
                    }))
                .layer(TimeoutLayer::with_status_code(StatusCode::REQUEST_TIMEOUT, Duration::from_secs(5)))
        )
}
