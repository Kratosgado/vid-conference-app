use actix_api::{DbConn, DbPool};

use super::{
    service,
    util::{LoginUser, SignUpUser},
};
use actix_web::{delete, get, post, web, HttpResponse};

#[post("/signup")]
pub async fn sign_up(pool: web::Data<DbPool>, req: web::Json<SignUpUser>) -> HttpResponse {
    log::info!("signing up user");
    let mut conn: DbConn = pool.get().expect("couldn't get db connection from pool");

    service::sign_up(&mut conn, req.into_inner()).await
}

#[post("/login")]
pub async fn login(pool: web::Data<DbPool>, req: web::Json<LoginUser>) -> HttpResponse {
    log::info!("logging in user");
    let mut conn: DbConn = pool.get().expect("couldn't get db connection from pool");

    service::login(&mut conn, req.into_inner()).await
}

#[get("/")]
pub async fn get_users(pool: web::Data<DbPool>) -> HttpResponse {
    log::info!("getting all users");

    let mut conn: DbConn = pool.get().expect("couldn't get db connection from pool");
    service::get_users(&mut conn).await
}

#[get("/{id}")]
pub async fn get_user(pool: web::Data<DbPool>, user_id: web::Path<String>) -> HttpResponse {
    log::info!("getting user by user id: {}", user_id);
    let mut conn: DbConn = pool.get().expect("couldn't get db connection from pool");

    service::get_user_by_id(&mut conn, user_id.into_inner()).await
}

// #[put("/{id}")]
// pub async fn update_user(
//     pool: web::Data<DbPool>,
//     web::Path(id): web::Path<String>,
//     req: web::Json<SignUpUser>,
// ) -> HttpResponse {
//     log::info!("updating user by id: {}", id);
//     let mut conn: DbConn = pool.get().expect("couldn't get db connection from pool");

//     service::update_user(&mut conn, id, req.into_inner()).await
// }

#[delete("/{id}")]
pub async fn delete_user(pool: web::Data<DbPool>, user_id: web::Path<String>) -> HttpResponse {
    log::info!("deleting user by id: {:?}", user_id);
    service::delete_user(pool, user_id.into_inner()).await
}
