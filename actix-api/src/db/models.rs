use crate::schema::users;
use actix_jwt_auth_middleware::FromRequest;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Queryable, Insertable, Selectable)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: String,
    pub username: String,
    pub email: String,
    pub password: String,
    pub salt: String,
}
