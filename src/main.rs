use std::str::FromStr;
use tracing_subscriber::{EnvFilter, fmt, prelude::*};
use crate::routes::build_routes;
use crate::state::AuroriteState;

mod database;
mod routes;
mod state;
mod config;

const ENV_FILTER: &str = "vismut_core=DEBUG,aurorite=DEBUG";

#[tokio::main]
async fn main() -> () {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::try_from_env("AURORITE_LOG").unwrap_or(
            EnvFilter::from_str(ENV_FILTER).unwrap()
        ))
        .init();

    let state = AuroriteState::new().await;
    let listener = tokio::net::TcpListener::bind(state.get_env().host()).await.unwrap();
    tracing::info!("Aurorite is ready. Listening on {}", listener.local_addr().unwrap());

    let app = build_routes().with_state(state);
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
