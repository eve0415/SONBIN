pub mod memory;
pub mod redis;

use crate::error::OAuth2Error;
use async_trait::async_trait;

const STATE_LIFETIME: u64 = 300;

#[async_trait]
pub trait SecurityManager: Send + Sync {
    async fn save_state(&mut self, state: String, code_verifier: String)
        -> Result<(), OAuth2Error>;
    async fn verify_state(&mut self, state: &str) -> Result<String, OAuth2Error>;
}
