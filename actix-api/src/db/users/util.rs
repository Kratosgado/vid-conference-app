use serde::Deserialize;

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
