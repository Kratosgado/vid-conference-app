use actix_api::{DbManager, DbPool};
use diesel::{r2d2, PgConnection};

pub mod auth;
pub mod models;
pub mod users;

pub fn create_pool() -> DbPool {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager: DbManager = r2d2::ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool");
    pool
}
