use crate::responses::{AuroriteErrorResponse, CharacterInfo, ClientCharacters, FullCharacterBaseInfo};
use crate::database::Character;
use crate::requests::PostCharacterBase;
use crate::extractors::AuthorizedClient;
use crate::state::AuroriteState;
use axum::extract::{State, Json, Path};
use axum::http::StatusCode;
use axum::routing::get;
use axum::Router;
use crate::responses::FailableResponse;
use crate::traits::IntoJson;
use crate::utils::uuid::EncodedUuid;

async fn get_characters(
    State(state): State<AuroriteState>,
    AuthorizedClient(client): AuthorizedClient,
) -> FailableResponse<ClientCharacters> {
    match Character::filter_by_client_id(client.id).exec(&mut state.db()).await {
        Ok(character_records) => {
            let mut characters = Vec::with_capacity(character_records.len());
            for record in &character_records {
                let result = CharacterInfo::try_from(record);
                if result.is_ok() {
                    characters.push(result.ok().unwrap())
                } else {
                    return Err((StatusCode::INTERNAL_SERVER_ERROR, result.err().unwrap().json()))
                }
            }
            Ok((StatusCode::OK, ClientCharacters { characters }.json()))
        },
        Err(err) => Err((StatusCode::INTERNAL_SERVER_ERROR, AuroriteErrorResponse::new(err).json()))
    }
}

async fn post_character(
    State(state): State<AuroriteState>,
    AuthorizedClient(client): AuthorizedClient,
    Json(body): Json<PostCharacterBase>
) -> FailableResponse<FullCharacterBaseInfo> {
    let mut db = state.db();
    let record = Character::create()
        .client(client)
        .name(body.name)
        .full_name(body.full_name)
        .strength(body.strength)
        .intelligence(body.intelligence)
        .wisdom(body.wisdom)
        .dexterity(body.dexterity)
        .constitution(body.constitution)
        .charisma(body.charisma)
        .level(body.level)
        .exec(&mut db)
        .await;
    match record {
        Ok(ref record) => {
            match FullCharacterBaseInfo::try_from(record) {
                Ok(info) => Ok((StatusCode::CREATED, info.json())),
                Err(err) => Err((StatusCode::INTERNAL_SERVER_ERROR, err.json()))
            }
        },
        Err(err) => Err((StatusCode::INTERNAL_SERVER_ERROR, AuroriteErrorResponse::new(err).json()))
    }

}

async fn get_character(
    Path(EncodedUuid(character_id)): Path<EncodedUuid>,
    State(state): State<AuroriteState>,
    AuthorizedClient(client): AuthorizedClient,
) -> FailableResponse<FullCharacterBaseInfo> {
    match Character::get_by_id(&mut state.db(), character_id).await {
        Ok(ref record) => {
            if record.client_id == client.id {
                match FullCharacterBaseInfo::try_from(record) {
                    Ok(info) => Ok((StatusCode::OK, info.json())),
                    Err(err) => Err((StatusCode::INTERNAL_SERVER_ERROR, err.json()))
                }
            } else {
                Err((StatusCode::NOT_FOUND, AuroriteErrorResponse::new("character not found").json()))
            }
        },
        Err(err) => {
            if err.is_record_not_found() {
                Err((StatusCode::NOT_FOUND, AuroriteErrorResponse::new("character not found").json()))
            } else {
                Err((StatusCode::INTERNAL_SERVER_ERROR, AuroriteErrorResponse::new(err).json()))
            }
        }
    }
}

pub fn build_characters_routes() -> Router<AuroriteState> {
    Router::new()
        .route("/", get(get_characters).post(post_character))
        .route("/{character_id}", get(get_character))
}
