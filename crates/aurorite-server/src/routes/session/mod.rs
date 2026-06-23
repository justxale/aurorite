use axum::Router;
use crate::AuroriteState;

mod root;
mod characters;

pub fn build_sessions_routes() -> Router<AuroriteState> {
    let to_nest = root::build_root_routes()
        .nest("/characters", characters::build_character_routes());
    Router::new()
        .nest("/sessions", to_nest)
}