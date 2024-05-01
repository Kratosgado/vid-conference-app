use crate::db::auth;

use super::service;
use actix_api::DbPool;
use actix_web::{delete, get, post, web, HttpRequest, HttpResponse};
use types::users::{LoginUser, Role, SignUpUser};

#[post("/signup")]
pub async fn sign_up(pool: web::Data<DbPool>, req: web::Json<SignUpUser>) -> HttpResponse {
    log::info!("signing up user");

    service::sign_up(pool, req.into_inner()).await
}

#[post("/login")]
pub async fn login(pool: web::Data<DbPool>, req: web::Json<LoginUser>) -> HttpResponse {
    log::info!("logging in user");
    service::login(pool, req.into_inner()).await
}

#[get("/")]
pub async fn get_users(req: HttpRequest, pool: web::Data<DbPool>) -> HttpResponse {
    log::info!("getting all users");
    match auth::authenticate(req).await {
        Ok(claim) => {
            log::info!("role: {:?}", claim.role.clone());
            if Role::Admin == claim.role {
                return service::get_users(pool).await;
            }
            HttpResponse::Unauthorized().finish()
        }
        Err(err) => HttpResponse::from_error(err),
    }
}

#[get("/{id}")]
pub async fn get_user(pool: web::Data<DbPool>, user_id: web::Path<String>) -> HttpResponse {
    log::info!("getting user by user id: {}", user_id);

    service::get_user_by_id(pool, user_id.into_inner()).await
}

#[delete("/{id}")]
pub async fn delete_user(pool: web::Data<DbPool>, user_id: web::Path<String>) -> HttpResponse {
    log::info!("deleting user by id: {:?}", user_id);
    service::delete_user(pool, user_id.into_inner()).await
}
