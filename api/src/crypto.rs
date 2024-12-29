use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use base64::Engine;
use ring::rand::{SecureRandom, SystemRandom};

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
