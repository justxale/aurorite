use aurorite_dataflow::database::Client;
use crate::requests::{ClientAuth, NewClientData, UpdatedClientData};
use crate::responses::ClientToken;
use crate::responses::{AuroriteErrorResponse, ClientInfo, FailableResponse};
use crate::state::AuroriteState;
use crate::traits::IntoJson;
use aurorite_util::auth::{generate_password, hash_password, verify};
use aurorite_util::jwt::encode_key;
use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::{get, post};
use axum::{Json, Router};
use crate::extractors::AuthorizedUnchecked;

#[tracing::instrument]
async fn get_self(
    State(state): State<AuroriteState>,
    AuthorizedUnchecked(user): AuthorizedUnchecked,
) -> FailableResponse<ClientInfo> {
    match Client::get_by_id(&mut state.db(), user.id()).await {
        Err(_) => Err((
            StatusCode::NOT_FOUND,
            Json(AuroriteErrorResponse::new(format!(
                "user {} not found",
                user.id()
            ))),
        )),
        Ok(record) => Ok((
            StatusCode::OK,
            Json(ClientInfo {
                display_name: record.display_name,
                username: record.username,
            }),
        )),
    }
}

#[tracing::instrument]
async fn edit_self(
    State(state): State<AuroriteState>,
    AuthorizedUnchecked(user): AuthorizedUnchecked,
    Json(fields): Json<UpdatedClientData>,
) -> FailableResponse<ClientInfo> {
    let mut db = state.db();
    let record = Client::get_by_id(&mut db, user.id()).await;
    if record.is_err() {
        return Err((
            StatusCode::NOT_FOUND,
            Json(AuroriteErrorResponse::new(format!(
                "user {} not found",
                user.id()
            ))),
        ));
    }
    let mut record = record.unwrap();
    let UpdatedClientData {
        display_name,
        is_master: _,
        is_admin: _,
    } = fields;
    if let Some(new_name) = display_name
        && record
            .update()
            .display_name(new_name)
            .exec(&mut db)
            .await
            .is_err()
    {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            AuroriteErrorResponse::new("failed to update").json(),
        ));
    }
    Ok((
        StatusCode::OK,
        Json(ClientInfo {
            display_name: record.display_name,
            username: record.username,
        }),
    ))
}

#[tracing::instrument]
async fn login_client(
    State(state): State<AuroriteState>,
    Json(body): Json<ClientAuth>,
) -> FailableResponse<ClientToken> {
    let record = Client::get_by_username(&mut state.db(), &body.login).await;
    if record.is_err() {
        return Err((
            StatusCode::NOT_FOUND,
            Json(AuroriteErrorResponse::new(format!(
                "user {} not found",
                body.login
            ))),
        ));
    }
    let record = record.unwrap();
    if verify(&body.password, &record.pwd) {
        let access = encode_key(record.id);
        match access {
            Err(_) => Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(AuroriteErrorResponse::new("error while creating token")),
            )),
            Ok(access) => Ok((
                StatusCode::OK,
                Json(ClientToken {
                    token_type: String::from("bearer"),
                    access_token: access,
                }),
            )),
        }
    } else {
        Err((
            StatusCode::NOT_FOUND,
            Json(AuroriteErrorResponse::new(format!(
                "user {} not found",
                body.login
            ))),
        ))
    }
}

#[tracing::instrument]
async fn register_client(
    State(state): State<AuroriteState>,
    AuthorizedUnchecked(user): AuthorizedUnchecked,
    Json(body): Json<NewClientData>,
) -> FailableResponse<ClientInfo> {
    let mut db = state.db();
    if Client::get_by_username(&mut db, &body.nickname)
        .await
        .is_ok()
    {
        return Err((
            StatusCode::CONFLICT,
            AuroriteErrorResponse::new("client already exists").json(),
        ));
    }
    let mut record = Client::create()
        .username(body.nickname)
        .display_name(body.display_name);
    if let Some(ref pwd) = body.password {
        record = record.pwd(hash_password(pwd).unwrap());
    } else {
        record = record.pwd(hash_password(&generate_password()).unwrap());
    }
    let record = record.exec(&mut db).await.unwrap();
    Ok((
        StatusCode::CREATED,
        Json(ClientInfo {
            username: record.username,
            display_name: record.display_name,
        }),
    ))
}

pub fn build_client_routes() -> Router<AuroriteState> {
    Router::new()
        .route("/me", get(get_self).put(edit_self))
        .route("/auth/login", post(login_client))
        .route("/auth/register", post(register_client))
}
