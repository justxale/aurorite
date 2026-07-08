mod agsp;
mod backgrounds;
mod campaigns;
mod characters;
mod classes;
mod client;
mod races;
mod rolls;
mod session;

use crate::responses::AuroriteErrorResponse;
use crate::state::AuroriteState;
use crate::traits::IntoJson;
use aurorite_util::common::create_hex;
use aurorite_util::env;
use axum::error_handling::HandleErrorLayer;
use axum::extract::{MatchedPath, Request};
use axum::http::{header, Method};
use axum::response::Response;
use axum::routing::any;
use axum::{http, BoxError, Router};
use http::StatusCode;
use std::time::Duration;
use tower::{buffer::BufferLayer, limit::RateLimitLayer, ServiceBuilder};
use tower_http::cors::{AllowOrigin, CorsLayer};
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
        .nest("/rolls", rolls::build_roll_routes())
        .nest("/sessions", session::build_sessions_routes())
        .route("/healthcheck", any(async || StatusCode::NO_CONTENT))
        .route_service("/", ServeDir::new("static"))
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(|err: BoxError| async move {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        AuroriteErrorResponse::new(err).json(),
                    )
                }))
                .layer(BufferLayer::new(1024))
                .layer(RateLimitLayer::new(128, Duration::from_mins(1)))
                .layer(
                    TraceLayer::new_for_http()
                        .make_span_with(|request: &Request<_>| {
                            let matched_path = request
                                .extensions()
                                .get::<MatchedPath>()
                                .map(MatchedPath::as_str);
                            tracing::info_span!(
                                "request",
                                method = ?request.method(),
                                matched_path,
                                request_id = create_hex::<8>()
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
                ))
                .option_layer(if env().allow_cors {
                    Some(
                        CorsLayer::new()
                            .allow_methods([Method::GET, Method::POST])
                            .allow_origin(AllowOrigin::exact("http://localhost:3000".parse().unwrap()))
                            .allow_credentials(true)
                            .allow_headers([header::AUTHORIZATION, header::ACCEPT]),
                    )
                } else {
                    None
                }),
        )
}
