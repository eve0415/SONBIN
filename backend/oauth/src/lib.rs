pub mod error;
pub mod security;

use crate::security::SecurityManager;
use base64::Engine;
use rand::random;
use reqwest::ClientBuilder;
use reqwest::{Client as HttpClient, StatusCode};
use serde::{Deserialize, Serialize};
use serenity::all::UserId;
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use url::Url;

const AUTHORIZATION_URL: &str = "https://discord.com/oauth2/authorize";
const DISCORD_API_URL: &str = "https://discord.com/api/v10";
const DISCORD_CDN_URL: &str = "https://cdn.discordapp.com";
const RESPONSE_TYPE: &str = "code";
const SCOPE: &str = "identify guilds.members.read";
const CODE_CHALLENGE_METHOD: &str = "S256";
const GRANT_TYPE: &str = "authorization_code";
const GUILD_ID: &str = "1176516474102353950";

#[derive(Clone)]
pub struct DiscordOAuth {
    id: String,
    secret: String,
    redirect_url: String,
    http_client: HttpClient,
    security_manager: Arc<Mutex<dyn SecurityManager>>,
}

impl DiscordOAuth {
    pub fn new(
        id: String,
        secret: String,
        redirect_url: String,
        security_manager: Arc<Mutex<dyn SecurityManager>>,
    ) -> Box<Self> {
        Box::new(DiscordOAuth {
            id,
            secret,
            redirect_url,
            http_client: ClientBuilder::new().https_only(true).build().unwrap(),
            security_manager,
        })
    }

    pub async fn generate_authorization_url(self) -> error::Result<Url> {
        let (state, code_verifier, code_challenge) = generate_state_and_code_challenge();

        self.security_manager
            .lock()
            .await
            .save_state(state.clone(), code_verifier.clone())
            .await?;

        let url = Url::parse_with_params(
            AUTHORIZATION_URL,
            &[
                ("client_id", self.id.to_owned()),
                ("response_type", RESPONSE_TYPE.to_string()),
                ("scope", SCOPE.to_string()),
                ("redirect_uri", self.redirect_url.to_string()),
                ("state", state),
                ("code_challenge", code_challenge),
                ("code_challenge_method", CODE_CHALLENGE_METHOD.to_string()),
                ("prompt", "none".to_string()),
            ],
        )
        .map_err(|e| error::Error::Unknown(e.into()))?;

        Ok(url)
    }

    pub async fn get_user(self, code: String, state: String) -> error::Result<User> {
        let code_verifier = self
            .security_manager
            .lock()
            .await
            .verify_state(&state)
            .await?;

        let params = HashMap::from([
            ("client_id", self.id.to_owned()),
            ("client_secret", self.secret.to_owned()),
            ("grant_type", GRANT_TYPE.to_string()),
            ("code", code),
            ("redirect_uri", self.redirect_url.to_string()),
            ("code_verifier", code_verifier),
        ]);

        let response = self
            .http_client
            .post(format!("{DISCORD_API_URL}/oauth2/token"))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .form(&params)
            .send()
            .await
            .map_err(|e| {
                log::error!("{:?}", e);

                error::Error::Unknown(e.into())
            })?;

        if response.status() != StatusCode::OK {
            log::error!("Failed to get access token: {:?}", response.text().await);

            return Err(error::Error::Unknown(anyhow::anyhow!(
                "Failed to get access token"
            )));
        }

        let res = response.json::<AccessTokenResponse>().await.map_err(|e| {
            log::error!("{:?}", e);

            error::Error::Unknown(e.into())
        })?;

        let response = self
            .http_client
            .get(format!(
                "{DISCORD_API_URL}/users/@me/guilds/{GUILD_ID}/member"
            ))
            .bearer_auth(res.access_token)
            .send()
            .await
            .map_err(|e| error::Error::Unknown(e.into()))?;

        if response.status() != StatusCode::OK {
            log::error!("Failed to get guild member: {:?}", response.text().await);

            return Err(error::Error::NotMember);
        }

        let member = response
            .json::<serenity::model::guild::Member>()
            .await
            .map_err(|e| error::Error::Unknown(e.into()))?;

        Ok(User {
            id: member.user.id,
            name: member.user.name,
            avatar: member.user.avatar.as_ref().map_or_else(
                || {
                    format!(
                        "{DISCORD_CDN_URL}/embed/avatars/{}.png",
                        member.user.id.get() % 6
                    )
                },
                |avatar| {
                    format!(
                        "{DISCORD_CDN_URL}/avatars/{user}/{hash}.{ext}",
                        user = member.user.id,
                        hash = avatar,
                        ext = if avatar.is_animated() { "gif" } else { "webp" },
                    )
                },
            ),
        })
    }
}

fn generate_state_and_code_challenge() -> (String, String, String) {
    let state = generate_random_string(16);
    let code_verifier = generate_random_string(32);
    let code_challenge = base64::engine::general_purpose::URL_SAFE_NO_PAD
        .encode(Sha256::digest(code_verifier.as_bytes()));

    (state, code_verifier, code_challenge)
}

fn generate_random_string(size: usize) -> String {
    let random_bytes: Vec<u8> = (0..size).map(|_| random::<u8>()).collect();

    base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(&random_bytes)
}

/// Response from Discord's OAuth2 token endpoint
/// [Access Token Response](https://discord.com/developers/docs/topics/oauth2#authorization-code-grant-access-token-response)
#[derive(Deserialize, Debug)]
struct AccessTokenResponse {
    pub(crate) access_token: String,
    // token_type: String,
    // expires_in: i32,
    // refresh_token: String,
    // scope: String,
}

#[derive(Serialize, Debug)]
pub struct User {
    pub id: UserId,
    pub name: String,
    pub avatar: String,
}

#[cfg(test)]
mod tests {
    use crate::generate_state_and_code_challenge;
    use base64::Engine;
    use sha2::{Digest, Sha256};

    #[test]
    fn it_should_verify_code_challenge() {
        let (_, code_verifier, code_challenge) = generate_state_and_code_challenge();

        assert_eq!(
            base64::engine::general_purpose::URL_SAFE_NO_PAD
                .encode(Sha256::digest(code_verifier.as_bytes())),
            code_challenge
        );
    }
}
