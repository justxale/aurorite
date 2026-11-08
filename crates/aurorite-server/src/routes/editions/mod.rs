use crate::state::AuroriteState;
use utoipa_axum::router::OpenApiRouter;

mod root;
mod backgrounds;
mod classes;
mod races;

pub fn build_editions_routes() -> OpenApiRouter<AuroriteState> {
    let id_router = OpenApiRouter::new()
        .nest("/backgrounds", backgrounds::build_backgrounds_routes())
        .nest("/classes", classes::build_classes_routes())
        .nest("/races", races::build_races_routes());
    OpenApiRouter::new()
        .nest("/{edition_id}", id_router)
        .merge(root::build_root_routes())
}