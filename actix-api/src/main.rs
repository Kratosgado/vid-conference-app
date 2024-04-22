use actix_web::{get, middleware, web, App, HttpResponse, HttpServer};
use diesel::{pg, r2d2, PgConnection};
use std::io;

pub mod db;
pub mod schema;

pub type DbPool = diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<diesel::PgConnection>>;
pub type DbConn = diesel::r2d2::PooledConnection<diesel::r2d2::ConnectionManager<diesel::PgConnection>>;
type DbManager = diesel::r2d2::ConnectionManager<diesel::PgConnection>;

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
            .wrap(middleware::Logger::default())
            .service(hello)
            .service(web::scope("users").configure(db::users::user_config))
            .default_service(web::route().to(|| HttpResponse::NotFound()))
    })
    .bind((host.clone(), port.clone()))
    .unwrap();
    log::info!("Starting server at http://{}:{}", host, port);

    server.run().await.expect("Failed to start server");
}

#[get("/")]
async fn hello() -> HttpResponse {
    HttpResponse::Ok().body("Hello world!")
}
