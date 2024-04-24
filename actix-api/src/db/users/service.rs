// The service module is where the business logic of the application is implemented.use actix_api::DbConn;

use crate::db::{models::User, users::util::Role};

use super::util::{LoginUser, SignUpUser};
use crate::schema::users::dsl::*;
use actix_api::{DbConn, DbPool};
use actix_session::Session;
use actix_web::{web, HttpResponse};
use diesel::{delete, prelude::*};

pub async fn sign_up(pool: &web::Data<DbPool>, sign_up_data: SignUpUser) -> HttpResponse {
    log::info!("creating user with data: {:?}", sign_up_data.clone());

    let (password_hash, salt_str) = crate::db::auth::hash_password(&sign_up_data.password);

    let new_user = User {
        id: uuid::Uuid::new_v4().to_string(),
        username: sign_up_data.username.clone(),
        email: sign_up_data.email.clone(),
        password: password_hash,
        salt: salt_str,
    };

    let mut conn: DbConn = pool.get().unwrap();

    let res = diesel::insert_into(users)
        .values(&new_user)
        .execute(&mut conn);

    match res {
        Ok(_) => {
            log::info!("user created successfully");
            HttpResponse::Created().finish()
        }
        Err(err) => {
            log::error!("{:?}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}

// create function for login
pub async fn login(
    session: Session,
    pool: &web::Data<DbPool>,
    login_data: LoginUser,
) -> HttpResponse {
    log::info!("finding user with email: {}", login_data.email.clone());
    let mut conn: DbConn = pool.get().unwrap();

    let res: Result<User, _> = users
        .filter(email.eq(&login_data.email))
        .first::<User>(&mut conn);

    match res {
        Ok(user) => {
            use crate::db::auth::verify_password;

            log::info!("user found...checking password");
            if verify_password(&login_data.password, &user.password) {
                if user.username == "admin" {
                    session.insert("admin", Role::Admin).unwrap();
                } else {
                    session.insert(user.id, Role::User).unwrap();
                }
                log::info!("user logged in successfully");
                return HttpResponse::Ok().json({
                    crate::db::auth::generate_token(user.username.clone())
                });
            }
            log::warn!("cannot log in user");
            HttpResponse::Unauthorized().finish()
        }
        Err(err) => {
            log::error!("{:?}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub async fn get_users( pool: &web::Data<DbPool>) -> HttpResponse {
    
    let mut conn: DbConn = pool.get().unwrap();

    log::info!("getting all users");
    let res = users.load::<User>(&mut conn);

    match res {
        Ok(rusers) => {
            log::info!("users retrieved: {:?}", rusers.len());
            HttpResponse::Ok().json(rusers)
        }
        Err(err) => {
            log::error!("{:?}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub async fn get_user_by_id(pool: web::Data<DbPool>, user_id: String) -> HttpResponse {
    let user = web::block(move || {
        let mut conn: DbConn = pool.get().unwrap();

        let user = users.find(user_id).get_result::<User>(&mut conn);
        user
    })
    .await
    .unwrap();
    match user {
        Ok(user) => {
            log::info!("user found");
            HttpResponse::Ok().json(user)
        }
        Err(err) => {
            log::error!("User not found: {:?}", err);
            HttpResponse::NotFound().finish()
        }
    }
}

pub async fn delete_user(pool: &web::Data<DbPool>, user_id: String) -> HttpResponse {
    let mut conn: DbConn = pool.get().unwrap();

    let count = delete(users.find(user_id)).execute(&mut conn);

    match count {
        Ok(_) => {
            log::info!("user deleted successfully");
            HttpResponse::Ok().finish()
        }
        Err(err) => {
            log::error!("{:?}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}
