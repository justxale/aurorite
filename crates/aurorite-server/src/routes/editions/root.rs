use axum::Router;
use axum::routing::get;
use crate::state::AuroriteState;

async fn get_editions() {
    
}

async fn post_edition() {
    
}

pub fn build_root_routes() -> Router<AuroriteState> {
    Router::new()
        .route("/", get(get_editions).post(post_edition))
}