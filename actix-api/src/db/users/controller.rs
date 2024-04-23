use actix_api::{DbConn, DbPool};

use super::{
    service,
    util::{LoginUser, SignUpUser},
};
use actix_web::{post, web, HttpResponse};

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
