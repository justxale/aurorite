use crate::responses::{FailableResponse, RollResult, SessionCharacters};
use crate::state::AuroriteState;
use crate::traits::IntoJson;
use aurorite_dataflow::enums::{Ability, Skill};
use aurorite_runtime::Character;
use aurorite_util::uuid::EncodedUuid;
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use serde::Deserialize;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

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

#[utoipa::path(
    get,
    path = "/",
    params(("session_id", description = "ID of session")),
    tag = "sessions",
    responses((status = OK, description = "List session characters"), (status = NOT_FOUND, body = crate::responses::AuroriteErrorResponse))
)]
async fn get_session_characters(
    State(state): State<AuroriteState>,
    Path(params): Path<PathParams>,
) -> FailableResponse<SessionCharacters> {
    state.session_and(params.session_id.uuid(), |v| {
        let characters = v.characters().values().cloned().collect();
        Ok((StatusCode::OK, SessionCharacters { characters }.json()))
    })?    
}

#[utoipa::path(
    get,
    path = "/{character_id}",
    params(
        ("session_id", description = "ID of session"),
        ("character_id", description = "ID of character")
    ),
    tag = "sessions",
    responses((status = OK, description = "Session character"), (status = NOT_FOUND, body = crate::responses::AuroriteErrorResponse))
)]
async fn get_session_character(
    State(state): State<AuroriteState>,
    Path(params): Path<PathParams>,
) -> FailableResponse<Character> {
    let char = state
        .session_character_and(params.session_id.uuid(), params.character_id.uuid(), |v| {
            v.clone()
        })?;
    Ok((StatusCode::OK, char.json()))
}

#[utoipa::path(
    get,
    path = "/{character_id}/roll",
    params(
        ("session_id", description = "ID of session"),
        ("character_id", description = "ID of character"),
        ("attr" = String, Query, description = "Skill or ability to roll"),
        ("save" = Option<bool>, Query, description = "Roll a saving throw")
    ),
    tag = "sessions",
    responses((status = OK, description = "Character roll result"), (status = NOT_FOUND, body = crate::responses::AuroriteErrorResponse))
)]
async fn get_character_roll(
    State(state): State<AuroriteState>,
    Path(params): Path<PathParams>,
    Query(query): Query<ApiRollQuery>,
) -> FailableResponse<RollResult> {
    let dice = state
        .session_character_and(
            params.session_id.uuid(),
            params.character_id.uuid(),
            |v| match (query.attr, query.save) {
                (Either::Left(skill), _) => v.skill_dice(skill),
                (Either::Right(ability), Some(false)) | (Either::Right(ability), None) => {
                    v.ability_dice(ability)
                }
                (Either::Right(ability), Some(true)) => v.save_throw_dice(ability),
            },
        )?;
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

pub fn build_character_routes() -> OpenApiRouter<AuroriteState> {
    OpenApiRouter::new()
        .routes(routes!(get_session_characters))
        .routes(routes!(get_session_character))
        .routes(routes!(get_character_roll))
}
