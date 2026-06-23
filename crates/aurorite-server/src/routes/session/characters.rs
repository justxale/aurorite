use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::Router;
use axum::routing::{any, get, post};
use serde::Deserialize;
use aurorite_dataflow::enums::{Ability, Skill};
use aurorite_util::formulas::DiceRollResult;
use aurorite_util::uuid::EncodedUuid;
use crate::responses::{AuroriteErrorResponse, FailableResponse, RollResult, SessionCharacters};
use crate::session::character::Character;
use crate::state::AuroriteState;
use crate::traits::IntoJson;

#[derive(Deserialize)]
struct PathParams {
    session_id: EncodedUuid,
    character_id: EncodedUuid
}

#[derive(Deserialize)]
#[serde(untagged)]
enum Either<T1, T2> {
    Left(T1),
    Right(T2),
}

#[derive(Deserialize)]
struct ApiRollQuery {
    attr: Either<Skill, Ability>,
    save: Option<bool>
}

async fn get_session_characters(
    State(state): State<AuroriteState>,
    Path(params): Path<PathParams>
) -> FailableResponse<SessionCharacters> {
    let characters: Vec<Character> = if let Some(ref session) = state.manager.session(params.session_id.uuid())
    {
        session.characters().iter().map(|v| v.value().clone()).collect()
    } else {
        return Err((StatusCode::NOT_FOUND, AuroriteErrorResponse::new("no character with this id").json()))
    };
    Ok((StatusCode::OK, SessionCharacters { characters }.json()))
}

async fn get_session_character(
    State(state): State<AuroriteState>,
    Path(params): Path<PathParams>
) -> FailableResponse<Character> {
    let char = if let Some(session) = state.manager.session(params.session_id.uuid())
        && let Some(character) = session.character(params.character_id.uuid())
    {
        character.value().clone()
    } else {
        return Err((StatusCode::NOT_FOUND, AuroriteErrorResponse::new("no character with this id").json()))
    };
    Ok((StatusCode::OK, char.json()))
}

async fn get_character_roll(
    State(state): State<AuroriteState>,
    Path(params): Path<PathParams>,
    Query(query): Query<ApiRollQuery>
) -> FailableResponse<RollResult> {
    let dice = if let Some(session) = state.manager.session(params.session_id.uuid())
        && let Some(character) = session.character(params.character_id.uuid())
    {
        match (query.attr, query.save) {
            (Either::Left(skill), _) => character.skill_dice(skill),
            (Either::Right(ability), Some(false)) | (Either::Right(ability), None) => character.ability_dice(ability),
            (Either::Right(ability), Some(true)) => character.save_throw_dice(ability),
        }
    } else {
        return Err((StatusCode::NOT_FOUND, AuroriteErrorResponse::new("no character with this id").json()))
    };
    Ok((StatusCode::OK, RollResult { parsed_amount: dice.amount, parsed_dice: dice.max, parsed_bonus: dice.bonus, result: dice.roll() }.json() ))
}

pub fn build_character_routes() -> Router<AuroriteState> {
    let id_router = Router::new()
        .route("/", get(get_session_character))
        .route("/roll", get(get_character_roll));
    Router::new()
        .nest("/{character_id}", id_router)
        .route("/", get(get_session_characters))
}