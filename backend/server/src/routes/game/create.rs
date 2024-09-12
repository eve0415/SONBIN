use crate::error::{AppError, ResponseResult};
use crate::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use game::config::{GameMode, GameSettings};
use game::errors::Error;
use game::game::Game;
use serde::Deserialize;
use serenity::all::UserId;

pub(crate) async fn new_game(
    State(mut state): State<AppState>,
    axum::extract::Json(data): axum::extract::Json<NewGameRequest>,
) -> ResponseResult<axum::response::Json<Game>> {
    match state
        .manager
        .create_game(data.host, data.mode, data.settings)
    {
        Ok(game) => Ok(axum::response::Json(game)),

        Err(e) => match e {
            Error::OngoingGame { host, game_id } => Err(AppError {
                status: StatusCode::CONFLICT,
                message: Some(format!(
                    "User {} has an ongoing game with id {}",
                    host, game_id
                )),
            }),

            _ => Err(AppError {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                message: None,
            }),
        },
    }
}

#[derive(Deserialize, Debug)]
pub(crate) struct NewGameRequest {
    host: UserId,
    mode: GameMode,
    settings: GameSettings,
}
