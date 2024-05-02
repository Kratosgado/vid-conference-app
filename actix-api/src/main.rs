use actix_api::DbPool;
use actix_cors::Cors;
// use actix_session::SessionMiddleware;
use actix_web::{web, App, HttpResponse, HttpServer};

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

    let pool: DbPool = db::create_pool();
    // let (redis_store , key) = db::create_redis_middleware().await;

    let server = HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            // .wrap(SessionMiddleware::new(redis_store.clone(), key.clone()))
            .app_data(web::Data::new(pool.clone()))
            .service(web::scope("/users").configure(db::users::user_config))
            .default_service(web::to(|| HttpResponse::Ok()))
    })
    .bind((host.clone(), port.clone()))
    .unwrap();
    log::info!("Starting server at http://{}:{}", host, port);

    server.run().await.expect("Failed to start server");
}
