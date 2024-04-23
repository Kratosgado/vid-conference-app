// The service module is where the business logic of the application is implemented.use actix_api::DbConn;

use crate::db::models::User;

use super::util::{LoginUser, SignUpUser};
use actix_api::{DbConn, DbPool};
use actix_web::{web, HttpResponse};
use diesel::{delete, prelude::*};
use crate::schema::users::dsl::*;


pub async fn sign_up(conn: &mut DbConn, sign_up_data: SignUpUser) -> HttpResponse {

    log::info!("creating user with data: {:?}", sign_up_data.clone());
    // let res = diesel::insert_into(users::table())
    //     .values(&sign_up_data)
    //     .execute(conn);
    let (password_hash, salt_str) = crate::db::auth::hash_password(&sign_up_data.password);

    let new_user = User {
        id: uuid::Uuid::new_v4().to_string(),
        username: sign_up_data.username.clone(),
        email: sign_up_data.email.clone(),
        password: password_hash,
        salt: salt_str,
    };

    let res = diesel::insert_into(users).values(&new_user).execute(conn);

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
pub async fn login(conn: &mut DbConn, login_data: LoginUser) -> HttpResponse {

    log::info!("finding user with email: {}", login_data.email.clone());
    let res: Result<User, _> = users
        .filter(email.eq(&login_data.email))
        .first::<User>(conn);

    match res {
        Ok(user) => {
            use crate::db::auth::verify_password;

            log::info!("user found...checking password");
            if verify_password(&login_data.password, &user.password) {
                log::info!("user logged in successfully");
                return HttpResponse::Ok().finish();
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

pub async fn get_users(conn: &mut DbConn) -> HttpResponse {

    log::info!("getting all users");
    let res = users.load::<User>(conn);

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

pub async fn get_user_by_id(conn: &mut DbConn, user_id: String) -> HttpResponse {
    let user = users.find(user_id).get_result::<User>(conn);
    match user {
        Ok(user ) => {
            log::info!("user found");
            HttpResponse::Ok().json(user)
        },
        Err(err) => {
            log::error!("User not found: {:?}", err);
            HttpResponse::NotFound().finish()
        }
    }
}

pub async fn delete_user(pool: web::Data<DbPool>, user_id: String) -> HttpResponse {

    let mut conn: DbConn = pool.get().unwrap();

    let count = delete(users.find(user_id)).execute(&mut conn);

    match count {
        Ok(_) => {
            log::info!("user deleted successfully");
            HttpResponse::Ok().finish()
        },
        Err(err ) => {
            log::error!("{:?}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}
