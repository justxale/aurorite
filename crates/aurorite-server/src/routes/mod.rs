mod agsp;
mod campaigns;
mod characters;
mod client;
mod editions;
mod rolls;
mod service;
mod session;
mod tracks;

use crate::state::AuroriteState;
use aurorite_util::common::create_hex;
use axum::extract::{MatchedPath, Request};
use axum::response::Response;
use axum::{Router, http};
use http::StatusCode;
use std::time::Duration;
use tower::ServiceBuilder;
use tower_http::services::ServeDir;
use tower_http::timeout::TimeoutLayer;
use tower_http::trace::TraceLayer;
use tracing::Span;
use utoipa::openapi::security::{Flow, OAuth2, Password, Scopes, SecurityScheme};
use utoipa::{Modify, OpenApi};
use utoipa_axum::router::OpenApiRouter;
use utoipa_rapidoc::RapiDoc;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    modifiers(&SecurityModifier),
    tags(
        (name = "clients"),
        (name = "tracks")
    )
)]
struct ApiDoc;

struct SecurityModifier;

impl Modify for SecurityModifier {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "password",
                SecurityScheme::OAuth2(OAuth2::with_description(
                    [Flow::Password(Password::new(
                        "/clients/auth/login",
                        Scopes::new(),
                    ))],
                    "Authorization with login + password",
                )),
            )
        }
    }
}

const RAPIDOC_HTML: &str = include_str!("../../static/rapidoc.html");

pub fn build_routes() -> Router<AuroriteState> {
    let router = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .merge(tracks::build_track_routes())
        .merge(client::build_client_routes())
        .route_service("/", ServeDir::new("static"));
    let (router, api) = router.split_for_parts();
    router
        .nest("/characters", characters::build_characters_routes())
        .nest("/editions", editions::build_editions_routes())
        .nest("/campaigns", campaigns::build_campaign_routes())
        .nest("/agsp", agsp::build_agsp_routes())
        .nest("/rolls", rolls::build_roll_routes())
        .nest("/sessions", session::build_sessions_routes())
        .nest("/service", service::build_service_routes())
        .merge(SwaggerUi::new("/swagger").url("/openapi.json", api))
        .merge(
            RapiDoc::new("openapi.json")
                .path("/docs")
                .custom_html(RAPIDOC_HTML),
        )
        .layer(
            ServiceBuilder::new()
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
                )),
        )
}
