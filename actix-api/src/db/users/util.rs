use crate::schema::users;
use diesel::prelude::*;
use serde::Deserialize;

#[derive(Deserialize, Insertable)]
#[diesel(table_name = users)]
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
