use actix_api::{DbConn, DbPool};
use actix_session::Session;

use crate::db::users::util::Role;

use super::{
    service,
    util::{LoginUser, SignUpUser},
};
use actix_web::{delete, get, post, web, HttpResponse};

#[post("/signup")]
pub async fn sign_up(pool: web::Data<DbPool>, req: web::Json<SignUpUser>) -> HttpResponse {
    log::info!("signing up user");

    service::sign_up(&pool, req.into_inner()).await
}

#[post("/login")]
pub async fn login(
    session: Session,
    pool: web::Data<DbPool>,
    req: web::Json<LoginUser>,
) -> HttpResponse {
    log::info!("logging in user");
    service::login(session, &pool, req.into_inner()).await
}

#[get("/")]
pub async fn get_users(session: Session, pool: web::Data<DbPool>) -> HttpResponse {
    log::info!("getting all users");
    match session.get::<Role>("admin") {
        Ok(role) => {
            log::info!("role: {:?}", role.clone());
            if let Some(Role::Admin) = role {
                return service::get_users(&pool).await;
            }
            HttpResponse::Unauthorized().finish()
        }
        Err(_) => HttpResponse::Unauthorized().finish(),
    }
}

#[get("/{id}")]
pub async fn get_user(pool: web::Data<DbPool>, user_id: web::Path<String>) -> HttpResponse {
    log::info!("getting user by user id: {}", user_id);

    service::get_user_by_id(pool, user_id.into_inner()).await
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
    service::delete_user(&pool, user_id.into_inner()).await
}
