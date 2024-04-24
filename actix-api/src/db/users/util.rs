use jsonwebtoken::get_current_timestamp;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Clone, Debug)]
pub struct SignUpUser {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct LoginUser {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct UserClaims {
    pub iat: u64,
    pub exp: u64,
    pub email: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum Role {
    Admin,
    User,
}
