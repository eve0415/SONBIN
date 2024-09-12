use crate::error::Error;
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
    async fn save_state(&mut self, state: String, code_verifier: String) -> Result<(), Error> {
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

    async fn verify_state(&mut self, state: &str) -> Result<String, Error> {
        let mut lock = self.challenges.lock().unwrap();
        match lock.remove(state) {
            Some(code_verifier) => Ok(code_verifier),
            None => Err(Error::InvalidState {
                state: state.to_owned(),
            }),
        }
    }
}
