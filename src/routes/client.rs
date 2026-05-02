use crate::requests::{ClientAuth, NewClientData};
use crate::responses::ClientToken;
use crate::utils::jwt::{encode_key, Authorization};
use crate::state::AuroriteState;
use axum::{Json, Router};
use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::{get, post};
use crate::database::Client;
use crate::responses::{AuroriteErrorResponse, ClientInfo, FailableResponse};
use crate::utils::auth::{generate_password, hash_password, verify};

#[tracing::instrument]
async fn get_client(State(state): State<AuroriteState>, user: Authorization) -> FailableResponse<ClientInfo> {
    match Client::get_by_id(&mut state.db(), user.id()).await {
        Err(_) => Err((
            StatusCode::NOT_FOUND,
            Json(AuroriteErrorResponse::new(format!("user {} not found", user.id())))
        )),
        Ok(record) => Ok((
            StatusCode::OK,
            Json(ClientInfo { display_name: record.display_name, nickname: record.nickname })
        )),
    }
}

#[tracing::instrument]
async fn login_client(State(state): State<AuroriteState>, Json(body): Json<ClientAuth>) -> FailableResponse<ClientToken> {
    let record = Client::get_by_nickname(&mut state.db(), &body.login).await;
    if record.is_err() {
        return Err((
            StatusCode::NOT_FOUND,
            Json(AuroriteErrorResponse::new(format!("user {} not found", body.login))),
        ))
    }
    let record = record.unwrap();
    if verify(&body.password, &record.pwd) {
        let access = encode_key(record.id);
        match access {
            Err(_) => Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(AuroriteErrorResponse::new("error while creating token"))
            )),
            Ok(access) =>  Ok((
                StatusCode::OK,
                Json(ClientToken { token_type: "bearer", access_token: access })
            ))
        }
    } else {
        Err((
            StatusCode::NOT_FOUND,
            Json(AuroriteErrorResponse::new(format!("user {} not found", body.login))),
        ))
    }
}


async fn register_client(State(state): State<AuroriteState>, user: Authorization, Json(body): Json<NewClientData>) -> FailableResponse<ClientInfo> {
    let mut db = state.db();
    if Client::get_by_nickname(&mut db, &body.nickname).await.is_ok() {
        return Err((StatusCode::CONFLICT, AuroriteErrorResponse::new("client already exists").json()));
    }
    let mut record = Client::create().nickname(body.nickname).display_name(body.display_name);
    if let Some(ref pwd) = body.password {
        record = record.pwd(hash_password(pwd).unwrap());
    } else {
        record = record.pwd(hash_password(&generate_password()).unwrap());
    }
    let record = record.exec(&mut db).await.unwrap();
    Ok((
        StatusCode::CREATED,
        Json(ClientInfo { nickname: record.nickname, display_name: record.display_name })
    ))
}

pub fn build_client_routes() -> Router<AuroriteState> {
    Router::new()
        .route("/me", get(get_client))
        .route("/auth/login", post(login_client))
        .route("/auth/register", post(register_client))
}
