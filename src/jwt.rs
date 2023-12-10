use jsonwebtoken::{ encode, Header, EncodingKey, Algorithm };
use serde::Serialize;
use std::time::{ SystemTime, UNIX_EPOCH };

use crate::types::credentials::GoogleCredentials;

/// Claims struct for JWT.
///
/// This struct represents the claims to be encoded in the JSON Web Token (JWT).
/// It includes issuer (`iss`), scope (`scope`), audience (`aud`), expiration time (`exp`),
/// and issued at time (`iat`).
#[derive(Serialize)]
struct Claims {
    iss: String,
    scope: String,
    aud: String,
    exp: usize,
    iat: usize,
}

/// Creates a JWT (JSON Web Token) for authentication with Google services.
///
/// This function generates a JWT using the provided Google credentials.
/// The token is valid for one hour from the time of creation.
///
/// ## Arguments
///
/// * `credentials` - A reference to [GoogleCredentials] containing the necessary credentials
///   for the JWT.
///
/// ## Returns
///
/// This function returns a [Result<String, Box<dyn std::error::Error + Sync + Send + 'static>>].
/// On success, it returns the JWT as a `String`. On failure, it returns an error.
///
/// ## Example
///
/// ```
/// let credentials = GoogleCredentials::new(client_email, private_key, token_uri);
/// let jwt = create_jwt(&credentials).expect("Failed to create JWT");
/// ```
pub fn create_jwt(
    credentials: &GoogleCredentials
) -> Result<String, Box<dyn std::error::Error + Sync + Send + 'static>> {
    let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() as usize;
    let expiration = now + 3600; // Token valid for 1 hour

    let claims = Claims {
        iss: credentials.client_email.clone(),
        scope: "https://www.googleapis.com/auth/cloud-platform".to_string(),
        aud: credentials.token_uri.clone(),
        exp: expiration,
        iat: now,
    };

    let encoding_key = EncodingKey::from_rsa_pem(credentials.private_key.as_bytes())?;

    let token = encode(&Header::new(Algorithm::RS256), &claims, &encoding_key)?;

    Ok(token)
}
