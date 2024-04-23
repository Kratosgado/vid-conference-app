pub mod controller;
pub mod service;
pub mod util;

use actix_web::web;

pub fn user_config(cfg: &mut web::ServiceConfig) {
    cfg.service(controller::sign_up).service(controller::login);
}