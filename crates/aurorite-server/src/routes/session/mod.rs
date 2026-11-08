use crate::AuroriteState;
use utoipa_axum::router::OpenApiRouter;

mod characters;
mod root;
mod initiative;

pub fn build_sessions_routes() -> OpenApiRouter<AuroriteState> {
    let router = OpenApiRouter::new()
        .merge(root::build_root_routes())
        .nest("/characters", characters::build_character_routes())
        .nest("/initiative", initiative::build_initiative_routes());
    OpenApiRouter::new().nest("/{session_id}", router)
}
