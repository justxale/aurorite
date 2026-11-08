use crate::state::AuroriteState;
use utoipa_axum::router::OpenApiRouter;

pub fn build_races_routes() -> OpenApiRouter<AuroriteState> {
    OpenApiRouter::new()
}
