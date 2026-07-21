use crate::responses::{AuroriteErrorResponse, FailableResponse, RollResult, SessionCharacters};
use crate::state::AuroriteState;
use crate::traits::IntoJson;
use aurorite_dataflow::enums::{Ability, Skill};
use aurorite_runtime::Character;
use aurorite_util::uuid::EncodedUuid;
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::routing::get;
use axum::Router;
use serde::Deserialize;

#[derive(Deserialize)]
struct PathParams {
    session_id: EncodedUuid,
    character_id: EncodedUuid,
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
    save: Option<bool>,
}

async fn get_session_characters(
    State(state): State<AuroriteState>,
    Path(params): Path<PathParams>,
) -> FailableResponse<SessionCharacters> {
    let characters: Vec<Character> =
        if let Some(ref session) = state.manager.session(params.session_id.uuid()) {
            session
                .ctx()
                .lock()
                .characters()
                .values()
                .cloned()
                .collect()
        } else {
            return Err((
                StatusCode::NOT_FOUND,
                AuroriteErrorResponse::new("no character with this id").json(),
            ));
        };
    Ok((StatusCode::OK, SessionCharacters { characters }.json()))
}

async fn get_session_character(
    State(state): State<AuroriteState>,
    Path(params): Path<PathParams>,
) -> FailableResponse<Character> {
    let char = state.session_character_and(
        params.session_id.uuid(),
        params.character_id.uuid(),
        |v| v.clone(),
    ).await?;
    Ok((StatusCode::OK, char.json()))
}

async fn get_character_roll(
    State(state): State<AuroriteState>,
    Path(params): Path<PathParams>,
    Query(query): Query<ApiRollQuery>,
) -> FailableResponse<RollResult> {
    let dice = state.session_character_and(
        params.session_id.uuid(),
        params.character_id.uuid(),
        |v| match (query.attr, query.save) {
            (Either::Left(skill), _) => v.skill_dice(skill),
            (Either::Right(ability), Some(false)) | (Either::Right(ability), None) => {
                v.ability_dice(ability)
            }
            (Either::Right(ability), Some(true)) => v.save_throw_dice(ability),
        },
    ).await?;
    Ok((
        StatusCode::OK,
        RollResult {
            parsed_amount: dice.amount,
            parsed_dice: dice.max,
            parsed_bonus: dice.bonus,
            result: dice.roll(),
        }
        .json(),
    ))
}

pub fn build_character_routes() -> Router<AuroriteState> {
    let id_router = Router::new()
        .route("/", get(get_session_character))
        .route("/roll", get(get_character_roll));
    Router::new()
        .nest("/{character_id}", id_router)
        .route("/", get(get_session_characters))
}
