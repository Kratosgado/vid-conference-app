// The service module is where the business logic of the application is implemented.use actix_api::DbConn;

use crate::db::models::User;

use super::util::{LoginUser, SignUpUser};
use actix_api::DbConn;
use actix_web::HttpResponse;
use diesel::{associations::HasTable, prelude::*};

pub async fn sign_up(conn: &mut DbConn, sign_up_data: SignUpUser) -> HttpResponse {
    use crate::schema::users::dsl::*;

    log::info!("creating user with data: {:?}", sign_up_data.clone());
    let res = diesel::insert_into(users::table())
        .values(&sign_up_data)
        .execute(conn);

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
    use crate::schema::users::dsl::*;

    log::info!("finding user with email: {}", login_data.email.clone());
    let res: Result<User, _> = users
        .filter(email.eq(&login_data.email))
        .first::<User>(conn);

    match res {
        Ok(user) => {
            log::info!("user found...checking password");
            if user.password == login_data.password {
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
