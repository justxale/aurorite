use aurorite_dataflow::database::{Background, Character, Class, Race, Db};
use crate::extractors::AuthorizedClient;
use crate::requests::{
    PostCharacterBase, PutCharacterBackground, PutCharacterClass, PutCharacterRace,
};
use crate::responses::FailableResponse;
use crate::responses::{AuroriteErrorResponse, ClientCharacters, CharacterInfo};
use crate::state::AuroriteState;
use crate::traits::IntoJson;
use aurorite_util::uuid::EncodedUuid;
use axum::Router;
use axum::extract::{Json, Path, State};
use axum::http::StatusCode;
use axum::routing::get;
use uuid::Uuid;
use aurorite_dataflow::dto::{BackgroundDto, CharacterDto, ClassDto, RaceDto};

async fn get_characters(
    State(state): State<AuroriteState>,
    AuthorizedClient(client): AuthorizedClient,
) -> FailableResponse<ClientCharacters> {
    match Character::filter_by_client_id(client.id)
        .exec(&mut state.db())
        .await
    {
        Ok(character_records) => {
            let mut characters = Vec::with_capacity(character_records.len());
            for record in &character_records {
                let result = CharacterInfo::try_from(record);
                if result.is_ok() {
                    characters.push(result.ok().unwrap())
                } else {
                    return Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        result.err().unwrap().json(),
                    ));
                }
            }
            Ok((StatusCode::OK, ClientCharacters { characters }.json()))
        }
        Err(err) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            AuroriteErrorResponse::new(err).json(),
        )),
    }
}

async fn post_character(
    State(state): State<AuroriteState>,
    AuthorizedClient(client): AuthorizedClient,
    Json(body): Json<PostCharacterBase>,
) -> FailableResponse<CharacterDto> {
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
        Ok(ref record) => match CharacterDto::try_from(record) {
            Ok(info) => Ok((StatusCode::CREATED, info.json())),
            Err(err) => Err((StatusCode::INTERNAL_SERVER_ERROR, AuroriteErrorResponse::new(err).json())),
        },
        Err(err) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            AuroriteErrorResponse::new(err).json(),
        )),
    }
}

async fn get_character(
    Path(EncodedUuid(character_id)): Path<EncodedUuid>,
    State(state): State<AuroriteState>,
    AuthorizedClient(client): AuthorizedClient,
) -> FailableResponse<CharacterDto> {
    match Character::filter_by_id(character_id)
        .filter_by_client_id(client.id)
        .get(&mut state.db())
        .await
    {
        Ok(ref record) => match CharacterDto::try_from(record) {
            Ok(info) => Ok((StatusCode::OK, info.json())),
            Err(err) => Err((StatusCode::INTERNAL_SERVER_ERROR, AuroriteErrorResponse::new(err).json())),
        },
        Err(err) => {
            if err.is_record_not_found() {
                Err((
                    StatusCode::NOT_FOUND,
                    AuroriteErrorResponse::new("character not found").json(),
                ))
            } else {
                Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    AuroriteErrorResponse::new(err).json(),
                ))
            }
        }
    }
}

async fn get_character_record(
    db: &mut Db,
    client_id: Uuid,
    character_id: Uuid,
) -> Result<Character, (StatusCode, Json<AuroriteErrorResponse>)> {
    Character::filter_by_client_id(client_id)
        .filter_by_id(character_id)
        .include(Character::fields().class())
        .include(Character::fields().race())
        .include(Character::fields().background())
        .get(db)
        .await
        .map_err(|err| {
            (
                StatusCode::NOT_FOUND,
                AuroriteErrorResponse::new(err).json(),
            )
        })
}

async fn get_character_class(
    Path(EncodedUuid(character_id)): Path<EncodedUuid>,
    State(state): State<AuroriteState>,
    AuthorizedClient(client): AuthorizedClient,
) -> FailableResponse<Option<ClassDto>> {
    let mut db = state.db();
    let record = Character::filter_by_client_id(client.id)
        .filter_by_id(character_id)
        .include(Character::fields().class())
        .get(&mut db)
        .await
        .map_err(|err| {
            (
                StatusCode::NOT_FOUND,
                AuroriteErrorResponse::new(err).json(),
            )
        })?;
    Ok((
        StatusCode::OK,
        record.class.get().as_ref().map(ClassDto::from).json(),
    ))
}

async fn put_character_class(
    Path(EncodedUuid(character_id)): Path<EncodedUuid>,
    State(state): State<AuroriteState>,
    AuthorizedClient(client): AuthorizedClient,
    Json(body): Json<PutCharacterClass>,
) -> FailableResponse<CharacterDto> {
    let mut db = state.db();
    let mut record = get_character_record(&mut db, client.id, character_id).await?;
    let mut dynamic = record.dyn_data.clone();
    dynamic.get_or_insert_default().chosen_class_skills = body.chosen_skills;

    match Class::get_by_id(&mut db, body.class_id.uuid()).await {
        Ok(ref class_record) => {
            record
                .update()
                .class(class_record)
                .dyn_data(dynamic)
                .exec(&mut db)
                .await
                .map_err(|err| {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        AuroriteErrorResponse::new(err).json(),
                    )
                })?;
            let response = CharacterDto::try_from(&record)
                .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, AuroriteErrorResponse::new(err).json()))?;
            Ok((StatusCode::OK, response.json()))
        }
        Err(err) => Err((
            StatusCode::NOT_FOUND,
            AuroriteErrorResponse::new(err).json(),
        )),
    }
}

async fn delete_character_class(
    Path(EncodedUuid(character_id)): Path<EncodedUuid>,
    State(state): State<AuroriteState>,
    AuthorizedClient(client): AuthorizedClient,
) -> FailableResponse<CharacterDto> {
    let mut db = state.db();
    let mut record = get_character_record(&mut db, client.id, character_id).await?;
    match record.update().class(None).exec(&mut db).await {
        Ok(_) => CharacterDto::try_from(&record)
            .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, AuroriteErrorResponse::new(err).json()))
            .map(|record| (StatusCode::OK, record.json())),
        Err(err) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            AuroriteErrorResponse::new(err).json(),
        )),
    }
}

async fn get_character_race(
    Path(EncodedUuid(character_id)): Path<EncodedUuid>,
    State(state): State<AuroriteState>,
    AuthorizedClient(client): AuthorizedClient,
) -> FailableResponse<Option<RaceDto>> {
    let mut db = state.db();
    let record = Character::filter_by_client_id(client.id)
        .filter_by_id(character_id)
        .include(Character::fields().race())
        .get(&mut db)
        .await
        .map_err(|err| {
            (
                StatusCode::NOT_FOUND,
                AuroriteErrorResponse::new(err).json(),
            )
        })?;
    Ok((
        StatusCode::OK,
        record.race.get().as_ref().map(RaceDto::from).json(),
    ))
}

async fn put_character_race(
    Path(EncodedUuid(character_id)): Path<EncodedUuid>,
    State(state): State<AuroriteState>,
    AuthorizedClient(client): AuthorizedClient,
    Json(body): Json<PutCharacterRace>,
) -> FailableResponse<CharacterDto> {
    let mut db = state.db();
    let mut record = get_character_record(&mut db, client.id, character_id).await?;
    match Race::get_by_id(&mut db, body.race_id.uuid()).await {
        Ok(ref race_record) => {
            record
                .update()
                .race(race_record)
                .exec(&mut db)
                .await
                .map_err(|err| {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        AuroriteErrorResponse::new(err).json(),
                    )
                })?;
            let response = CharacterDto::try_from(&record)
                .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, AuroriteErrorResponse::new(err).json()))?;
            Ok((StatusCode::OK, response.json()))
        }
        Err(err) => Err((
            StatusCode::NOT_FOUND,
            AuroriteErrorResponse::new(err).json(),
        )),
    }
}

async fn delete_character_race(
    Path(EncodedUuid(character_id)): Path<EncodedUuid>,
    State(state): State<AuroriteState>,
    AuthorizedClient(client): AuthorizedClient,
) -> FailableResponse<CharacterDto> {
    let mut db = state.db();
    let mut record = get_character_record(&mut db, client.id, character_id).await?;
    match record.update().race(None).exec(&mut db).await {
        Ok(_) => CharacterDto::try_from(&record)
            .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, AuroriteErrorResponse::new(err).json()))
            .map(|record| (StatusCode::OK, record.json())),
        Err(err) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            AuroriteErrorResponse::new(err).json(),
        )),
    }
}
async fn get_character_background(
    Path(EncodedUuid(character_id)): Path<EncodedUuid>,
    State(state): State<AuroriteState>,
    AuthorizedClient(client): AuthorizedClient,
) -> FailableResponse<Option<BackgroundDto>> {
    let mut db = state.db();
    let record = Character::filter_by_client_id(client.id)
        .filter_by_id(character_id)
        .include(Character::fields().background())
        .get(&mut db)
        .await
        .map_err(|err| {
            (
                StatusCode::NOT_FOUND,
                AuroriteErrorResponse::new(err).json(),
            )
        })?;
    Ok((
        StatusCode::OK,
        record
            .background
            .get()
            .as_ref()
            .map(BackgroundDto::from)
            .json(),
    ))
}

async fn put_character_background(
    Path(EncodedUuid(character_id)): Path<EncodedUuid>,
    State(state): State<AuroriteState>,
    AuthorizedClient(client): AuthorizedClient,
    Json(body): Json<PutCharacterBackground>,
) -> FailableResponse<CharacterDto> {
    let mut db = state.db();
    let mut record = get_character_record(&mut db, client.id, character_id).await?;
    match Background::get_by_id(&mut db, body.background_id.uuid()).await {
        Ok(ref background_record) => {
            record
                .update()
                .background(background_record)
                .exec(&mut db)
                .await
                .map_err(|err| {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        AuroriteErrorResponse::new(err).json(),
                    )
                })?;
            let response = CharacterDto::try_from(&record)
                .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, AuroriteErrorResponse::new(err).json()))?;
            Ok((StatusCode::OK, response.json()))
        }
        Err(err) => Err((
            StatusCode::NOT_FOUND,
            AuroriteErrorResponse::new(err).json(),
        )),
    }
}

async fn delete_character_background(
    Path(EncodedUuid(character_id)): Path<EncodedUuid>,
    State(state): State<AuroriteState>,
    AuthorizedClient(client): AuthorizedClient,
) -> FailableResponse<CharacterDto> {
    let mut db = state.db();
    let mut record = get_character_record(&mut db, client.id, character_id).await?;
    match record.update().background(None).exec(&mut db).await {
        Ok(_) => CharacterDto::try_from(&record)
            .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, AuroriteErrorResponse::new(err).json()))
            .map(|record| (StatusCode::OK, record.json())),
        Err(err) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            AuroriteErrorResponse::new(err).json(),
        )),
    }
}

pub fn build_characters_routes() -> Router<AuroriteState> {
    Router::new()
        .route("/", get(get_characters).post(post_character))
        .route("/{character_id}", get(get_character))
        .route(
            "/{character_id}/class",
            get(get_character_class)
                .put(put_character_class)
                .delete(delete_character_class),
        )
        .route(
            "/{character_id}/race",
            get(get_character_race)
                .put(put_character_race)
                .delete(delete_character_race),
        )
        .route(
            "/{character_id}/background",
            get(get_character_background)
                .put(put_character_background)
                .delete(delete_character_background),
        )
}
