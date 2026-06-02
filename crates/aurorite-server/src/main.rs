use aurorite_util::env;
use crate::routes::build_routes;
use crate::state::AuroriteState;
use axum::Router;
#[cfg(target_os = "windows")]
use std::io::{Read, Write, stderr, stdin};
use std::str::FromStr;
use tracing::Level;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::{EnvFilter, Registry, fmt, prelude::*};

pub mod extractors;
pub mod requests;
pub mod responses;
mod routes;
mod state;
#[cfg(test)]
mod tests;
mod traits;

async fn build_app() -> Router {
    let state = AuroriteState::new().await;
    build_routes().with_state(state)
}

fn setup_tracing() -> (WorkerGuard, WorkerGuard, WorkerGuard) {
    let appender = RollingFileAppender::builder()
        .rotation(Rotation::DAILY)
        .filename_suffix("log")
        .latest_symlink("latest.log")
        .build(std::env::current_dir().unwrap().join("logs"))
        .expect("failed to build RollingFileAppender");
    let (writer, _guard) = tracing_appender::non_blocking(appender);
    let debug_appender = RollingFileAppender::builder()
        .rotation(Rotation::DAILY)
        .filename_prefix("debug")
        .filename_suffix("log")
        .latest_symlink("latest.log")
        .build(std::env::current_dir().unwrap().join("logs"))
        .expect("failed to build RollingFileAppender");
    let (debug_writer, _debug_guard) = tracing_appender::non_blocking(debug_appender);
    let (stderr_writer, _stderr_guard) = tracing_appender::non_blocking(std::io::stderr());

    Registry::default()
        .with(
            fmt::Layer::default()
                .with_writer(writer)
                .with_ansi(false)
                .with_filter(EnvFilter::from_str(&env().log).unwrap()),
        )
        .with(
            fmt::Layer::default()
                .with_writer(debug_writer.with_max_level(Level::DEBUG))
                .with_ansi(false),
        )
        .with(
            fmt::Layer::default()
                .with_writer(stderr_writer)
                .with_filter(EnvFilter::from_str(&env().log).unwrap()),
        )
        .init();

    (_guard, _debug_guard, _stderr_guard)
}

async fn serve() -> () {
    let span = tracing::info_span!("startup");
    let _enter = span.enter();
    tracing::info!("performing Aurorite startup...");

    let listener = tokio::net::TcpListener::bind(format!("{}:{}", env().host, env().port))
        .await
        .unwrap();

    let app = build_app().await;
    drop(_enter);
    tracing::info!(
        "Aurorite (v{}) is ready. Listening on {}",
        env!("CARGO_PKG_VERSION"),
        listener.local_addr().unwrap()
    );
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

#[tokio::main]
async fn main() {
    #[cfg(debug_assertions)]
    let _ = dotenvy::dotenv();
    let _guards = setup_tracing();

    // Not using env() here because of probability of wrong env configuration
    #[cfg(target_os = "windows")]
    if std::env::var("AURORITE_AUTOEXIT")
        .ok()
        .is_none_or(|v| v == "0")
    {
        std::panic::set_hook(Box::new(|info| {
            let mut out = stderr().lock();
            let mut input = stdin();
            out.write_fmt(format_args!(
                "Fatal error occured: {}\nPress any key to continue...\n",
                info
            ))
            .unwrap();
            out.flush().unwrap();
            input.read_exact(&mut [0; 1]).unwrap();
        }));
    }

    serve().await;
}
