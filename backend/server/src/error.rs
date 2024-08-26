use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

pub(crate) struct AppError {
    pub(crate) status: StatusCode,
    pub(crate) message: Option<String>,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            self.status,
            self.message
                .unwrap_or_else(|| self.status.canonical_reason().unwrap_or("").to_string()),
        )
            .into_response()
    }
}

pub(crate) type ResponseResult<T> = std::result::Result<T, AppError>;
