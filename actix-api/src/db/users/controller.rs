use crate::{DbConn, DbPool};

use super::{service, util::SignUpUser};
use actix_web::{post, web, HttpResponse};

#[post("/signup")]
pub async fn sign_up(pool: web::Data<DbPool>, req: web::Json<SignUpUser>) -> HttpResponse {
    log::info!("signing up user");
    let mut conn: DbConn = pool.get().expect("couldn't get db connection from pool");

    service::sign_up(&mut conn, req.into_inner()).await
}
