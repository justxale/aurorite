use crate::state::AuroriteState;
use axum::{Form, Router, Json};
use axum::extract::State;
use axum::routing::{get, post};
use crate::database::Character;
use crate::extractors::Authorization;
use crate::requests::PostCharacterBase;
use crate::responses::FullCharacterBaseInfo;

async fn get_character(State(state): State<AuroriteState>, user: Authorization) -> &'static str {
    "your character here"
}

/*async fn post_character<'a>(
    State(state): State<AuroriteState>,
    user: Authorization,
    Json(body): Json<PostCharacterBase>
) -> FullCharacterBaseInfo<'a> {
    let mut db = state.db();
    let record = Character::create()
        .;
    FullCharacterBaseInfo::from()
}*/

pub fn build_characters_routes() -> Router<AuroriteState> {
    Router::new()
        //.route("/", post(post_character))
        .route("/{id}", get(get_character))
}
