use axum::Router;
use crate::state::AuroriteState;

pub fn build_races_routes() -> Router<AuroriteState> {
    Router::new()
}
