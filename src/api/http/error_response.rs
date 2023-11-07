use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;

use crate::domain::error::Error;

#[derive(Serialize)]
pub struct ErrorMessage {
    error: String,
}

fn status_code(error: &Error) -> StatusCode {
    match error {
        Error::EmailTaken => StatusCode::CONFLICT,
        Error::InvalidPassword => StatusCode::UNAUTHORIZED,
        Error::Unexpected(_) => StatusCode::INTERNAL_SERVER_ERROR,
        Error::Validation(_) => StatusCode::UNPROCESSABLE_ENTITY,
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let status_code = status_code(&self);
        let message = ErrorMessage {
            error: self.to_string(),
        };
        (status_code, Json(message)).into_response()
    }
}
