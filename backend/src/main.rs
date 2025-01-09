use actix_web::{web, App, HttpServer};
use std::env;

mod routes;
mod models;
mod services;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("Starting InstantMon server...");

    HttpServer::new(move || {
        App::new()
            .service(
                web::scope("/api")
                    .configure(routes::config)
            )
    })
    .bind(("0.0.0.0", 22221))?
    .run()
    .await
} 