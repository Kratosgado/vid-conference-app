use actix_api::DbPool;
use actix_session::{storage::RedisSessionStore, Session, SessionMiddleware};
use actix_web::{cookie::Key, web, App, HttpResponse, HttpServer};

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
    let redis_url = std::env::var("REDIS_URL").expect("REDIS_URL must be set");

    let redis_store = RedisSessionStore::new(redis_url).await.unwrap();
    let secret_key = Key::generate();
    let pool: DbPool = db::create_pool();

    let server = HttpServer::new(move || {
        App::new()
            .wrap(SessionMiddleware::new(
                redis_store.clone(),
                secret_key.clone(),
            ))
            .app_data(web::Data::new(pool.clone()))
            // .wrap(middleware::Logger::new("\"%r\" %s %D"))
            .service(web::scope("/users").configure(db::users::user_config))
            .default_service(web::to(|| HttpResponse::Ok()))
    })
    .bind((host.clone(), port.clone()))
    .unwrap();
    log::info!("Starting server at http://{}:{}", host, port);

    server.run().await.expect("Failed to start server");
}
