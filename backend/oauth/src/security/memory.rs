use crate::error::OAuth2Error;
use crate::security::{SecurityManager, STATE_LIFETIME};
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Clone, Default)]
pub struct InMemorySecurityManager {
    challenges: Arc<Mutex<HashMap<String, String>>>,
}

#[async_trait]
impl SecurityManager for InMemorySecurityManager {
    async fn save_state(
        &mut self,
        state: String,
        code_verifier: String,
    ) -> Result<(), OAuth2Error> {
        let challenges = Arc::clone(&self.challenges);

        {
            let mut lock = challenges.lock().unwrap();
            lock.insert(state.clone(), code_verifier);
        }

        // Remove the challenge after 5 minutes
        tokio::spawn(async move {
            tokio::time::sleep(tokio::time::Duration::from_secs(STATE_LIFETIME)).await;
            let mut lock = challenges.lock().unwrap();
            lock.remove(&state);
        });

        Ok(())
    }

    async fn verify_state(&mut self, state: &str) -> Result<String, OAuth2Error> {
        let mut lock = self.challenges.lock().unwrap();
        match lock.remove(state) {
            Some(code_verifier) => Ok(code_verifier),
            None => Err(OAuth2Error::InvalidState {
                state: state.to_owned(),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_save_state() {
        let mut manager = InMemorySecurityManager::default();
        let state = "state".to_string();
        let code_verifier = "code_verifier".to_string();

        manager
            .save_state(state.clone(), code_verifier.clone())
            .await
            .unwrap();

        let challenges = manager.challenges.lock().unwrap();
        assert_eq!(challenges.get(&state), Some(&code_verifier));
    }

    #[tokio::test]
    async fn test_verify_state() {
        let mut manager = InMemorySecurityManager::default();
        let state = "state".to_string();
        let code_verifier = "code_verifier".to_string();

        manager
            .save_state(state.clone(), code_verifier.clone())
            .await
            .unwrap();

        assert_eq!(manager.verify_state(&state).await.unwrap(), code_verifier);
    }

    #[tokio::test]
    async fn test_verify_state_invalid() {
        let mut manager = InMemorySecurityManager::default();
        let state = "state".to_string();

        assert!(manager.verify_state(&state).await.is_err())
    }
}
