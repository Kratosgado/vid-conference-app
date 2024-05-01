use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct SignUpUser {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Serialize)]
pub struct LoginUser {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct UserClaims {
    pub iat: u64,
    pub exp: u64,
    pub email: String,
    pub role: Role,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub enum Role {
    Admin,
    User,
}
