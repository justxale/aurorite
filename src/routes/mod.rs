mod characters;

use axum::Router;
use crate::state::AuroriteState;

pub fn build_routes() -> Router<AuroriteState> {
    Router::new()
        .nest("/characters", characters::build_characters_routes())
}