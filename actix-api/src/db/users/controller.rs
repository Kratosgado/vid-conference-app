use super::{service, util::SignUpUser};
use actix_web::{post, web, HttpResponse};

#[post("/signup")]
pub async fn sign_up(req: web::Json<SignUpUser>) -> HttpResponse {
    service::sign_up(SignUpUser {
        username: req.username.clone(),
        email: req.email.clone(),
        password: req.password.clone(),
    })
    .await
}
