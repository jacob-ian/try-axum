use axum::{
    response::{IntoResponse, Response},
    Json,
};
use hyper::StatusCode;
use serde::Serialize;

pub enum Error {
    BadRequest(String),
}

impl Error {
    fn get_status_code(&self) -> StatusCode {
        return match self {
            Self::BadRequest(_) => StatusCode::BAD_REQUEST,
        };
    }

    fn get_body(&self) -> ErrorBody {
        let status = self.get_status_code();
        return match self {
            Self::BadRequest(m) => ErrorBody {
                error: status.canonical_reason().unwrap_or("Unknown").to_string(),
                error_description: m.to_string(),
            },
        };
    }
}

#[derive(Serialize)]
struct ErrorBody {
    error: String,
    error_description: String,
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        (self.get_status_code(), Json(self.get_body())).into_response()
    }
}
