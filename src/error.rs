use axum::{response::IntoResponse, Json, http::StatusCode};

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Custom error msg: {0}")]
    CustomStrError(&'static str),
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
    #[error(transparent)]
    UrlParseError(#[from] url::ParseError)
}

#[derive(Serialize)]
pub struct Message {
    err_msg: String
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::BAD_GATEWAY, Json(Message {err_msg: format!("{}", self)})).into_response()
    }
}