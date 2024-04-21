use actix_web::{get, web, App, HttpResponse, HttpServer};
use dotenvy::dotenv;
use std::env::var;
use std::io;

pub mod db;
#[actix_web::main]
async fn main() -> io::Result<()> {
    dotenv().ok();
    let host = var("HOST").expect("HOST must be set");
    let port = var("ACTIX_PORT")
        .expect("ACTIX_PORT must be set")
        .parse()
        .unwrap();
    HttpServer::new(move || App::new().service(hello))
        .bind((host, port))?
        .run()
        .await
}

#[get("/")]
async fn hello() -> HttpResponse {
    HttpResponse::Ok().body("Hello world!")
}
