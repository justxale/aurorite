use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum_typed_multipart::TypedMultipartError;
use serde::Serialize;
use crate::traits::IntoJson;

#[derive(Serialize)]
pub struct AuroriteErrorResponse {
    err: String,
    #[serde(skip)]
    code: Option<StatusCode>
}

impl AuroriteErrorResponse {
    pub fn new(err: impl ToString) -> Self {
        Self {
            err: err.to_string(),
            code: None
        }
    }

    pub fn with_code(code: StatusCode, err: impl ToString) -> Self {
        Self {
            err: err.to_string(),
            code: Some(code)
        }
    }
}

impl IntoResponse for AuroriteErrorResponse {
    fn into_response(self) -> Response {
        (self.code.unwrap_or(StatusCode::INTERNAL_SERVER_ERROR), self.json()).into_response()
    }
}

impl From<TypedMultipartError> for AuroriteErrorResponse {
    fn from(value: TypedMultipartError) -> Self {
        Self {
            err: value.to_string(),
            code: Some(StatusCode::BAD_REQUEST)
        }
    }
}

pub type FailableResponse<T> =
    Result<(StatusCode, Json<T>), (StatusCode, Json<AuroriteErrorResponse>)>;
