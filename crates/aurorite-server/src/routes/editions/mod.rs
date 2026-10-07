use axum::Router;
use crate::state::AuroriteState;

mod root;
mod backgrounds;
mod classes;
mod races;

pub fn build_editions_routes() -> Router<AuroriteState> {
    let id_router = Router::new()
        .nest("/backgrounds", backgrounds::build_backgrounds_routes())
        .nest("/classes", classes::build_classes_routes())
        .nest("/races", races::build_races_routes());
    Router::new()
        .nest("/{edition_id}/", id_router)
        .merge(root::build_root_routes())
}