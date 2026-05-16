use crate::state::AuroriteState;
use axum::{Form, Router};
use axum::extract::State;
use axum::routing::get;
use crate::extractors::Authorization;

async fn get_character(State(state): State<AuroriteState>, user: Authorization) -> &'static str {
    "your character here"
}

/*async fn post_character(State(state): State<AuroriteState>, ) -> &'static str {
    
}*/

pub fn build_characters_routes() -> Router<AuroriteState> {
    Router::new()
        .route("/{id}", get(get_character))
}
