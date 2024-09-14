use crate::security::{SecurityManager, STATE_LIFETIME};
use crate::OAuth2Error;
use async_trait::async_trait;
use redis::AsyncCommands;

#[derive(Clone)]
pub struct RedisSecurityManager {
    pub redis_client: redis::Client,
}

#[async_trait]
impl SecurityManager for RedisSecurityManager {
    /// Save the state and code verifier to Redis
    async fn save_state(
        &mut self,
        state: String,
        code_verifier: String,
    ) -> Result<(), OAuth2Error> {
        redis::cmd("HSETEX")
            .arg("oauth")
            .arg(STATE_LIFETIME)
            .arg(state)
            .arg(code_verifier)
            .exec_async(
                &mut self
                    .redis_client
                    .get_multiplexed_tokio_connection()
                    .await
                    .map_err(|_| OAuth2Error::RedisConnectionLost)?,
            )
            .await
            .map_err(OAuth2Error::RedisError)?;

        Ok(())
    }

    /// Verify the state and return the code verifier
    /// TODO: What if the state is not found?
    async fn verify_state(&mut self, state: &str) -> Result<String, OAuth2Error> {
        let mut conn = self
            .redis_client
            .get_multiplexed_tokio_connection()
            .await
            .map_err(|_| OAuth2Error::RedisConnectionLost)?;
        let code_verifier: String = redis::cmd("HGET")
            .arg("oauth")
            .arg(state.to_owned())
            .query_async(&mut conn)
            .await
            .map_err(|_| OAuth2Error::InvalidState {
                state: state.to_owned(),
            })?;

        let _: () = conn
            .hdel("oauth", state.to_owned())
            .await
            .map_err(OAuth2Error::RedisError)?;

        Ok(code_verifier)
    }
}
