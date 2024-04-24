use actix_api::{DbManager, DbPool};
use actix_session::{storage::RedisSessionStore, SessionMiddleware};
use diesel::{r2d2, PgConnection};
use redis::{Client, Connection};

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

pub async fn create_redis_middleware() -> (RedisSessionStore, actix_web::cookie::Key) {
    let redis_url = std::env::var("REDIS_URL").expect("REDIS_URL must be set");
    let redis_store = RedisSessionStore::new(redis_url)
        .await
        .expect("Failed to create Redis session store");
    let secret_key = actix_web::cookie::Key::generate();
    (redis_store, secret_key)
}
