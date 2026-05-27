use crate::responses::FailableResponse;
use crate::state::AuroriteState;
use aurorite_agsp::{MAX_PACKAGE_SIZE, export};
use axum::Router;
use axum::body::Body;
use axum::extract::{DefaultBodyLimit, Multipart, State};
use axum::http::{StatusCode, header};
use axum::response::IntoResponse;
use axum::routing::get;
use std::path::PathBuf;
use std::sync::LazyLock;
use tokio_util::io::ReaderStream;

static ASSETS_PATH: LazyLock<PathBuf> =
    LazyLock::new(|| std::env::current_dir().unwrap().join("assets"));

async fn get_package(State(_state): State<AuroriteState>) -> impl IntoResponse {
    let stream = ReaderStream::new(export(ASSETS_PATH.clone()).await);
    (
        StatusCode::OK,
        [(
            header::CONTENT_DISPOSITION,
            "attachment; filename=text.tar.zst",
        )],
        Body::from_stream(stream),
    )
}

async fn post_package(
    State(_state): State<AuroriteState>,
    mut multipart: Multipart,
) -> FailableResponse<()> {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let _name = field.name().unwrap().to_string();
    }

    Ok((StatusCode::NO_CONTENT, axum::Json(())))
}

pub fn build_agsp_routes() -> Router<AuroriteState> {
    Router::new()
        .route("/", get(get_package).post(post_package))
        .layer(DefaultBodyLimit::max(MAX_PACKAGE_SIZE))
}
