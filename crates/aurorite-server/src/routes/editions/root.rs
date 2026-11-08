use crate::state::AuroriteState;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

#[utoipa::path(
    get,
    path = "/",
    tag = "editions",
    responses((status = OK, description = "List editions"))
)]
async fn get_editions() {
    
}

#[utoipa::path(
    post,
    path = "/",
    tag = "editions",
    responses((status = OK, description = "Create an edition"))
)]
async fn post_edition() {
    
}

pub fn build_root_routes() -> OpenApiRouter<AuroriteState> {
    OpenApiRouter::new().routes(routes!(get_editions, post_edition))
}