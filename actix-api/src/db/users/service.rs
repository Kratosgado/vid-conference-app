// The service module is where the business logic of the application is implemented.use actix_api::DbConn;

use crate::db::{auth, models::User};

use crate::schema::users::dsl::*;
use actix_api::{DbConn, DbPool};
use actix_web::{web, HttpResponse};
use diesel::{delete, prelude::*};
use types::users::{LoginUser, Role, SignUpUser};

/// Create a new user. and inserts it to database.
///
/// returns a 201 Created response if successful.
///
/// returns a 500 Internal Server Error response if an error occurs.
/// Panics if .
pub async fn sign_up(pool: web::Data<DbPool>, sign_up_data: SignUpUser) -> HttpResponse {
    log::info!("creating user with data: {:?}", sign_up_data.clone());

    let (password_hash, _salt_str) = crate::db::auth::hash_password(&sign_up_data.password);

    let new_user = User {
        id: uuid::Uuid::new_v4().to_string(),
        username: sign_up_data.username.clone(),
        email: sign_up_data.email.clone(),
        password: password_hash,
    };
    let res = web::block(move || {
        let mut conn: DbConn = pool.get().unwrap();

        diesel::insert_into(users)
            .values(&new_user)
            .execute(&mut conn)
    })
    .await;
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
pub async fn login(pool: web::Data<DbPool>, login_data: LoginUser) -> HttpResponse {
    log::info!("finding user with email: {}", login_data.email.clone());
    let res = web::block(move || {
        let mut conn: DbConn = pool.get().unwrap();
        users
            .filter(email.eq(&login_data.email))
            .first::<User>(&mut conn)
    })
    .await;

    match res {
        Ok(ok_res) => {
            log::info!("user found...checking password");
            let user = ok_res.unwrap();
            if auth::verify_password(&login_data.password, &user.password) {
                let role = if user.username == "admin" {
                    Role::Admin
                } else {
                    Role::User
                };
                log::info!("user logged in successfully");
                match auth::generate_token(user.email, role) {
                    Ok(token) => return HttpResponse::Ok().body(token),
                    Err(err) => return HttpResponse::from_error(err),
                }
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

pub async fn get_users(pool: web::Data<DbPool>) -> HttpResponse {
    log::info!("getting all users");
    let res = web::block(move || {
        let mut conn: DbConn = pool.get().unwrap();
        users.load::<User>(&mut conn)
    })
    .await;

    match res {
        Ok(res) => {
            let rusers = res.unwrap();
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
    let res = web::block(move || {
        let mut conn: DbConn = pool.get().unwrap();

        let user = users.find(user_id).get_result::<User>(&mut conn);
        user
    })
    .await;
    match res {
        Ok(user) => {
            let user = user.unwrap();
            log::info!("user found");
            HttpResponse::Ok().json(user)
        }
        Err(err) => {
            log::error!("User not found: {:?}", err);
            HttpResponse::NotFound().finish()
        }
    }
}

pub async fn delete_user(pool: web::Data<DbPool>, user_id: String) -> HttpResponse {
    let count = web::block(move || {
        let mut conn: DbConn = pool.get().unwrap();

        delete(users.find(user_id)).execute(&mut conn)
    }).await;

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
