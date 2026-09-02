use crate::AuroriteState;
use axum::Router;

mod characters;
mod root;
mod initiative;

pub fn build_sessions_routes() -> Router<AuroriteState> {
    let router = Router::new()
        .merge(root::build_root_routes())
        .nest("/characters", characters::build_character_routes())
        .nest("/initiative", initiative::build_initiative_routes());
    Router::new().nest("/{session_id}", router)
}
