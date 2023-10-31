use axum::{
    response::{IntoResponse, Response},
    Json,
};
use hyper::StatusCode;
use serde::Serialize;

pub enum Error {
    BadRequest(String),
    NotFound(String),
}

#[derive(Serialize)]
struct ErrorBody {
    error: String,
    description: String,
}

impl Error {
    fn get_status_code(&self) -> StatusCode {
        return match self {
            Error::BadRequest(_) => StatusCode::BAD_REQUEST,
            Error::NotFound(_) => StatusCode::NOT_FOUND,
        };
    }

    fn get_body(&self) -> ErrorBody {
        return match self {
            Error::BadRequest(d) => ErrorBody {
                error: self.get_reason(),
                description: d.to_string(),
            },
            Error::NotFound(d) => ErrorBody {
                error: self.get_reason(),
                description: d.to_string(),
            },
        };
    }

    fn get_reason(&self) -> String {
        return self
            .get_status_code()
            .canonical_reason()
            .unwrap_or("Unknown")
            .to_string();
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        (self.get_status_code(), Json(self.get_body())).into_response()
    }
}
