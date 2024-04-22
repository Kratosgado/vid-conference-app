use super::util::SignUpUser;
use actix_web::HttpResponse;
use diesel::RunQueryDsl;


pub async fn sign_up(sign_up_data: SignUpUser) -> HttpResponse {
    use crate::schema::users;

    let conn = &mut crate::db::establish_connection();
    let res = diesel::insert_into(users::table).values(&sign_up_data)
        .execute(conn);

    match res {
        Ok(_) => HttpResponse::Created().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}