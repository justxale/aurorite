use axum::Json;
use axum::http::StatusCode;
use serde::Serialize;

#[derive(Serialize)]
pub struct AuroriteErrorResponse {
    err: String,
}

impl AuroriteErrorResponse {
    pub fn new(err: impl ToString) -> Self {
        Self {
            err: err.to_string(),
        }
    }

    pub fn json(self) -> Json<AuroriteErrorResponse> {
        Json(self)
    }
}

pub type FailableResponse<T> = Result<(StatusCode, Json<T>), (StatusCode, Json<AuroriteErrorResponse>)>;
