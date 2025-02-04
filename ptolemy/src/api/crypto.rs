use super::error::ApiError;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use base64::Engine;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use ring::{
    digest::{digest, SHA256},
    rand::{SecureRandom, SystemRandom},
};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use tracing::{error, info};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClaimType {
    UserJWT,
    ServiceAPIKeyJWT,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "id")]
pub enum AuthClaims {
    UserJWT(Uuid),
    ServiceApiKeyJWT(Uuid),
}

/// Generates a 32 byte api key and encodes it as a base64 string.
///
/// # Returns
///
/// Returns a base64 encoded string containing the generated api key.
pub async fn generate_api_key(prefix: &str) -> String {
    let rng = SystemRandom::new();
    let mut api_key = [0u8; 48];
    rng.fill(&mut api_key).unwrap();
    // encode api key as b64
    let encoded_api_key = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(api_key);

    format!("{}-{}", prefix, encoded_api_key)
}

#[derive(Debug, Clone)]
pub struct PasswordHandler {
    argon2: Argon2<'static>,
}

impl PasswordHandler {
    pub fn new() -> Self {
        let argon2 = Argon2::default();
        Self { argon2 }
    }

    pub fn hash_password(&self, password: &str) -> String {
        let salt = SaltString::generate(&mut OsRng);
        let hash = self
            .argon2
            .hash_password(password.as_bytes(), &salt)
            .unwrap();
        hash.to_string()
    }

    pub fn verify_password(&self, password: &str, hash: &str) -> bool {
        let parsed_hash = PasswordHash::new(hash).unwrap();
        self.argon2
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok()
    }
}

type ClaimsResult<T> = Result<T, ApiError>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Claims<T> {
    sub: T,
    claim_type: ClaimType,
    exp: usize,
    iat: usize,
}

#[allow(dead_code)]
impl<T: for<'de> Deserialize<'de> + Serialize + Clone> Claims<T>
where
    T: Clone + for<'de> Deserialize<'de> + Serialize,
{
    pub fn new(sub: T, claim_type: ClaimType, valid_for_secs: usize) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as usize;
        Self {
            sub,
            claim_type,
            exp: now + valid_for_secs,
            iat: now,
        }
    }

    pub fn claim_type(&self) -> &ClaimType {
        &self.claim_type
    }

    pub fn sub(&self) -> &T {
        &self.sub
    }

    pub fn exp(&self) -> &usize {
        &self.exp
    }

    pub fn iat(&self) -> &usize {
        &self.iat
    }

    pub fn is_expired(&self) -> bool {
        self.exp
            < SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs() as usize
    }

    pub fn generate_auth_token(&self, secret: &[u8]) -> ClaimsResult<String> {
        Ok(
            encode(&Header::default(), &self, &EncodingKey::from_secret(secret)).map_err(|e| {
                error!("Failed to generate auth token: {}", e);
                ApiError::InternalError
            })?,
        )
    }

    pub fn from_token(token: Option<String>, secret: &[u8]) -> ClaimsResult<Option<Self>> {
        let token = match token {
            Some(token) => token,
            None => return Ok(None),
        };

        let claims = decode::<Self>(
            &token,
            &DecodingKey::from_secret(secret),
            &Validation::default(),
        )
        .map_err(|e| {
            info!("Failed to decode auth token: {}", e);
            ApiError::AuthError("Invalid token".to_string())
        })?;

        Ok(Some(claims.claims))
    }
}

pub type UuidClaims = Claims<Uuid>;

pub trait GenerateSha256 {
    fn sha256(&self) -> Vec<u8>;
}

impl<'a> GenerateSha256 for &'a [u8] {
    fn sha256(&self) -> Vec<u8> {
        generate_sha256(self)
    }
}

impl GenerateSha256 for Uuid {
    fn sha256(&self) -> Vec<u8> {
        generate_sha256(self.as_bytes())
    }
}

impl GenerateSha256 for serde_json::Value {
    fn sha256(&self) -> Vec<u8> {
        generate_sha256(self.to_string().as_bytes())
    }
}

impl GenerateSha256 for String {
    fn sha256(&self) -> Vec<u8> {
        generate_sha256(self.as_bytes())
    }
}

pub fn generate_sha256(data: &[u8]) -> Vec<u8> {
    let digest = digest(&SHA256, data);
    digest.as_ref().to_vec()
}
