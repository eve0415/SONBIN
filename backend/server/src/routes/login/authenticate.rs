use crate::error::{AppError, ResponseResult};
use crate::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use oauth::error::OAuth2Error;
use oauth::User;
use serde::{Deserialize, Serialize};

pub(crate) async fn get_login_url(
    State(state): State<AppState>,
) -> ResponseResult<axum::response::Json<RedirectResponse>> {
    Ok(axum::response::Json(RedirectResponse {
        url: state
            .oauth
            .generate_authorization_url()
            .await
            .map_err(|_| AppError {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                message: None,
            })?
            .to_string(),
    }))
}

pub(crate) async fn login(
    State(state): State<AppState>,
    axum::extract::Json(data): axum::extract::Json<LoginRequest>,
) -> ResponseResult<axum::response::Json<User>> {
    match state.oauth.get_user(data.code, data.state).await {
        Ok(user) => Ok(axum::response::Json(user)),

        Err(e) => match e {
            OAuth2Error::InvalidState { state: _ } => Err(AppError {
                status: StatusCode::UNAUTHORIZED,
                message: None,
            }),

            OAuth2Error::NotMember => Err(AppError {
                status: StatusCode::FORBIDDEN,
                message: None,
            }),

            OAuth2Error::RedisConnectionLost => Err(AppError {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                message: None,
            }),

            OAuth2Error::RedisError(_) => Err(AppError {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                message: None,
            }),

            OAuth2Error::InternalError(_) => Err(AppError {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                message: None,
            }),

            OAuth2Error::Unknown(e) => {
                log::error!("Unknown error: {}", e);

                Err(AppError {
                    status: StatusCode::INTERNAL_SERVER_ERROR,
                    message: None,
                })
            }
        },
    }
}

#[derive(Serialize, Debug)]
pub(crate) struct RedirectResponse {
    url: String,
}

#[derive(Deserialize, Debug)]
pub(crate) struct LoginRequest {
    code: String,
    state: String,
}
