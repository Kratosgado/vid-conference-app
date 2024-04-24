
use actix_web::{dev::ServiceRequest, Error, HttpMessage};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash,PasswordVerifier, PasswordHasher, SaltString},
    Argon2,
};
use jsonwebtoken::{encode, DecodingKey, EncodingKey, Header, Validation};

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

pub fn generate_token(username: String) -> String {
    let secret = std::env::var("JWT_KEY").expect("JWT_KEY must be set");
    let expiration = std::env::var("JWT_EXPIRATION")
        .expect("JWT_EXPIRATION must be set")
        .parse::<usize>()
        .expect("JWT_EXPIRATION must be an integer");
    
    let claims = UserClaims {
        username: username,
        exp: expiration,
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_bytes())).unwrap()

}

pub fn authenticate(req: ServiceRequest) -> Result<ServiceRequest, Error> {
    let token = req.headers().get("Authorization").and_then(|header| {
        header.to_str().ok().map(|token| token.replace("Bearer ", ""))
    });

    if let Some(token ) = token {
        let secret = std::env::var("JWT_KEY").expect("JWT_KEY must be set");
        let validation = Validation::default();
        match jsonwebtoken::decode::<UserClaims>(&token, &DecodingKey::from_secret(secret.as_bytes()), &validation) {
            Ok(claims) => {
                req.extensions_mut().insert(claims.claims);
                Ok(req)
            },
            Err(_) => Err(actix_web::error::ErrorUnauthorized("Invalid token")),
        }
    } else {
        Err(actix_web::error::ErrorUnauthorized("Missing token"))
    }
    
}