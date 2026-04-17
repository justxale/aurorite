use axum::extract::State;
use axum::Router;
use axum::routing::get;
use crate::state::AuroriteState;

async fn get_character(State(state): State<AuroriteState>) -> &'static str {
    "Hello!"
}

pub fn build_characters_routes() -> Router<AuroriteState> {
    Router::new()
        .route("/character/{id}", get(get_character))
}