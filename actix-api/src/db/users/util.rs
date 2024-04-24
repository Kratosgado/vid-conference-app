use serde::{Deserialize, Serialize};
use actix_jwt_auth_middleware::FromRequest;

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

#[derive(Serialize, Deserialize, Clone, FromRequest)]
pub struct UserClaims {
    pub user_id: String,
    role: Role,
}

#[derive(Clone, Serialize, Deserialize, Debug,)]
enum Role {
    Admin,
    User,
}