use crate::config::env;
use crate::routes::build_routes;
use crate::state::AuroriteState;
use std::str::FromStr;
use axum::Router;
use tracing_subscriber::{EnvFilter, fmt, prelude::*};

mod config;
mod database;
mod enums;
pub mod extractors;
pub mod responses;
mod routes;
mod state;
pub mod utils;
pub mod requests;

const ENV_FILTER: &str = "vismut_core=DEBUG,aurorite=DEBUG";

async fn build_app() -> Router {
    let state = AuroriteState::new().await;
    build_routes().with_state(state)
}

#[tokio::main]
async fn main() -> () {
    #[cfg(debug_assertions)]
    let _ = dotenvy::dotenv();

    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(
            EnvFilter::try_from_env("AURORITE_LOG")
                .unwrap_or(EnvFilter::from_str(ENV_FILTER).unwrap()),
        )
        .init();
    let listener = tokio::net::TcpListener::bind(
        format!("{}:{}", env().host, env().port)
    ).await.unwrap();

    tracing::info!(
        "Aurorite (v{}) is ready. Listening on {}",
        env!("CARGO_PKG_VERSION"),
        listener.local_addr().unwrap()
    );

    let app = build_app().await;
    axum::serve(listener, app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {
            tracing::info!("received SIGTERM, graceful shutdown...");
        },
        _ = terminate => {
            tracing::info!("received SIGTERM, graceful shutdown...");
        },
    }
}
