#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Redis connection lost")]
    RedisConnectionLost,

    #[error("State not found: {state}")]
    InvalidState { state: String },

    #[error("Not a member in the guild")]
    NotMember,

    #[error("Unknown error: {0}")]
    Unknown(#[source] anyhow::Error),
}

pub(crate) type Result<T, E = Error> = std::result::Result<T, E>;
