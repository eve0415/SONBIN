use redis::RedisError;
use std::error::Error;
use url::ParseError;

#[derive(thiserror::Error, Debug)]
pub enum OAuth2Error {
    #[error("Redis connection lost")]
    RedisConnectionLost,

    #[error("State not found: {state}")]
    InvalidState { state: String },

    #[error("Not a member in the guild")]
    NotMember,

    #[error(transparent)]
    RedisError(#[from] RedisError),

    #[error(transparent)]
    InternalError(#[from] ParseError),

    #[error(transparent)]
    Unknown(#[from] Box<dyn Error + Sync + Send>),
}
