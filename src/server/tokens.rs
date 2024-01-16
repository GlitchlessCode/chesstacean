use anyhow::{Ok, Result};
use hmac::{Hmac, Mac};
use jwt::{SignWithKey, VerifyWithKey};
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use sha2::Sha512;

use super::{user::UserInfo, utils::get_timestamp};

pub struct TokenManager {
    key: Hmac<Sha512>,
}

impl TokenManager {
    /// Initializes a new `TokenManager` with a random key
    ///
    /// # Panics
    /// This function will panic if an `Hmac<Sha512>` instance cannot
    /// be created successfully
    pub fn new() -> Self {
        let mut rand = [0u8; 32];
        thread_rng().fill(&mut rand);
        Self {
            key: Hmac::new_from_slice(&rand).expect("Must be a valid set of random values"),
        }
    }

    /// Create a websocket token, and return the signed JWT
    pub fn create_ws_token(&self, user_info: UserInfo, session_id: String, ws_claim: String) -> Result<String> {
        let claims = WSClaims::new(user_info, session_id, ws_claim);
        let token = claims.sign_with_key(&self.key)?;
        Ok(token)
    }

    /// Attempt to parse and verify the given JWT
    ///
    /// Returns `Ok(WSClaims)` if valid
    ///
    /// Returns `Err(Error)` if invalid, or if it ran into a parsing error
    pub fn parse_ws_token(&self, token: String) -> Result<WSClaims> {
        let parsed: WSClaims = token.verify_with_key(&self.key)?;
        Ok(parsed)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WSClaims {
    iss: String,
    iat: u128,
    exp: u128,
    aud: String,
    pub sub: String,
    pub ws: String,
    pub us: UserInfo,
}

impl WSClaims {
    fn new(user_info: UserInfo, session_id: String, ws_claim: String) -> Self {
        let timestamp = get_timestamp();
        let expiry = timestamp + 30000;
        Self {
            iss: "chesstacean.ca".to_string(),
            iat: timestamp,
            exp: expiry,
            aud: "chesstacean.ca".to_string(),
            sub: session_id,
            ws: ws_claim,
            us: user_info,
        }
    }
    pub fn valid(&self) -> bool {
        self.exp > get_timestamp()
    }
}
