use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash,PasswordVerifier, PasswordHasher, SaltString},
    Argon2,
};

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
    // let salt_str = SaltString::from_b64(&salt).unwrap();
    let password_hash = PasswordHash::new(hash).unwrap();

    argon2
        .verify_password(password.as_bytes(), &password_hash)
        .is_ok()
}