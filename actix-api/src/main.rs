use actix_api::DbManager;
use actix_web::{get, web, App, HttpResponse, HttpServer};
use diesel::{ r2d2, PgConnection};

pub mod db;
pub mod schema;

#[actix_web::main]
async fn main() {
    std::env::set_var("RUST_LOG", "debug");
    tracing_subscriber::fmt::init();

    dotenvy::dotenv().ok();
    let host = std::env::var("HOST").expect("HOST must be set");
    let port: u16 = std::env::var("ACTIX_PORT")
        .expect("ACTIX_PORT must be set")
        .parse()
        .unwrap();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager: DbManager = r2d2::ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool");

    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            // .wrap(middleware::Logger::new("\"%r\" %s %D"))
            .service(hello)
            .service(web::scope("/users").configure(db::users::user_config))
            .default_service(web::route().to(|| HttpResponse::NotFound()))
    })
    .bind((host.clone(), port.clone()))
    .unwrap();
    log::info!("Starting server at http://{}:{}", host, port);

    server.run().await.expect("Failed to start server");
}

#[get("/")]
async fn hello() -> HttpResponse {
    log::info!("Hello world!");
    HttpResponse::Ok().body("Hello world!")
}
