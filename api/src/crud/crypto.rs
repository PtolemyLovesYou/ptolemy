use crate::error::CRUDError;
use crate::models::auth::crypto::{crypt, gen_salt};
use crate::state::DbConnection;
use base64::Engine;
use diesel_async::RunQueryDsl;
use ring::rand::{SecureRandom, SystemRandom};
use tracing::error;

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

/// Hashes a given password using a generated salt and returns the hashed password.
///
/// # Arguments
///
/// * `conn` - A mutable reference to the database connection.
/// * `password_str` - A string slice that holds the password to be hashed.
///
/// # Returns
///
/// Returns a `Result` containing the hashed password as a `String` if successful, or a `CRUDError` if an error occurs.
///
/// # Errors
///
/// This function will return `CRUDError::InsertError` if there is an error generating the salt or hashing the password.
pub async fn hash_password(
    conn: &mut DbConnection<'_>,
    password_str: &str,
) -> Result<(String, String), CRUDError> {
    let salt: String = match diesel::select(gen_salt("bf")).get_result(conn).await {
        Ok(s) => s,
        Err(e) => {
            error!("Unable to generate salt: {}", e);
            return Err(CRUDError::InsertError);
        }
    };

    let hashed_password: String = match diesel::select(crypt(password_str, salt.clone()))
        .get_result(conn)
        .await
    {
        Ok(s) => s,
        Err(e) => {
            error!("Unable to generate hashed password: {}", e);
            return Err(CRUDError::InsertError);
        }
    };

    Ok((hashed_password, salt))
}

pub async fn verify_password(
    conn: &mut DbConnection<'_>,
    password: &str,
    salt: &str,
    hashed_password: &str,
) -> Result<bool, CRUDError> {
    let att_hashed_password: String = match diesel::select(crypt(password, salt.to_string()))
        .get_result(conn)
        .await
    {
        Ok(s) => s,
        Err(e) => {
            error!("Unable to generate hashed password: {}", e);
            return Err(CRUDError::InsertError);
        }
    };

    Ok(att_hashed_password == hashed_password)
}
