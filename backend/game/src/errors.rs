use serenity::all::UserId;

#[derive(thiserror::Error, PartialEq, Debug)]
pub enum Error {
    #[error("Max players reached")]
    MaxPlayers,

    #[error("Cannot generate board")]
    BoardGenerationError,

    #[error("Game not found with ID {game_id}")]
    NotFound { game_id: u32 },

    #[error("User {host} already has an ongoing game with ID {game_id}")]
    OngoingGame { host: UserId, game_id: u32 },
}
