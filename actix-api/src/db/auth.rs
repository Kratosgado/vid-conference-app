use actix_web::{Error, HttpRequest};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use jsonwebtoken::{encode, get_current_timestamp, DecodingKey, EncodingKey, Header, Validation};

use super::users::util::UserClaims;

pub fn hash_password(password: &str) -> (String, String) {
    let salt_str = SaltString::generate(&mut OsRng);

    // argon2 with default params (Argon2id v19)
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt_str)
        .unwrap()
        .to_string();

    (password_hash, salt_str.to_string())
}

pub fn verify_password(password: &String, hash: &String) -> bool {
    let argon2 = Argon2::default();
    let password_hash = PasswordHash::new(hash).unwrap();

    argon2
        .verify_password(password.as_bytes(), &password_hash)
        .is_ok()
}

pub fn generate_token(email: String) -> Result<String, Error> {
    log::info!("generating token for user: {}", email);
    let secret = std::env::var("JWT_KEY").expect("JWT_KEY must be set");

    let iat = seconds_since_epoch();
    let exp = expiry(iat, SECONDS_VALID_FOR);

    let claims = UserClaims { email, exp, iat };
    log::info!("original iat: {}", claims.iat);
    log::info!("original exp: {}", claims.exp);

    let token = match encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    ) {
        Ok(token) => Ok(token),
        Err(err) => {
            log::error!("Failed to encode token. Error: {}", err.to_string());
            return Err(actix_web::error::ErrorUnauthorized(err.to_string()));
        }
    };

    token
}

pub async fn authenticate(req: HttpRequest) -> Result<UserClaims, Error> {
    log::info!("authenticating user");
    let token = req.headers().get("Authorization").and_then(|header| {
        header
            .to_str()
            .ok()
            .map(|token| token.replace("Bearer ", ""))
    });

    if let Some(token) = token {
        log::info!("token: {}", token);
        let secret = std::env::var("JWT_KEY").expect("JWT_KEY must be set");
        let mut validation = Validation::default();
        validation.leeway = 0;
        let sleep_time = std::time::Duration::from_secs(SECONDS_TO_SLEEP);
        std::thread::sleep(sleep_time);

        match jsonwebtoken::decode::<UserClaims>(
            &token,
            &DecodingKey::from_secret(secret.as_ref()),
            &validation,
        ) {
            Ok(claims) => Ok(claims.claims),
            Err(err) => {
                log::error!("Error decoding token: {:?}", err);
                Err(actix_web::error::ErrorUnauthorized(err.to_string()))
            }
        }
    } else {
        Err(actix_web::error::ErrorUnauthorized("Missing token"))
    }
}

pub const SECONDS_VALID_FOR: u64 = 3600;
pub const SECONDS_TO_SLEEP: u64 = 4;

pub const SECRET_KEY: &str = "007: The Spy Who Loved Me";

pub fn expiry(secs_since_epoch: u64, secs_valid_for: u64) -> u64 {
    secs_since_epoch + secs_valid_for
}

pub fn seconds_since_epoch() -> u64 {
    get_current_timestamp()
}
