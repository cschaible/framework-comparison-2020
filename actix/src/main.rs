#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
extern crate log;

use std::sync::Arc;

use actix_web::error::BlockingError;
use actix_web::{middleware, web, App, HttpServer};
use anyhow::format_err;
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use log::LevelFilter;
use log::{error, warn};
use r2d2::Pool;

use crate::footballer_repository::FootballerRepository;

pub mod footballer;
pub mod footballer_repository;
pub mod footballer_rest_controller;
pub mod schema;

embed_migrations!();

async fn run_db_migrations(
    pool: Arc<Pool<ConnectionManager<PgConnection>>>,
) -> Result<(), BlockingError<anyhow::Error>> {
    web::block(move || {
        warn!("Migrate Database");
        embedded_migrations::run_with_output(&*pool.get()?, &mut std::io::stdout())?;
        Ok(())
    })
    .await
}

#[actix_rt::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();

    // Configure logging
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "warn")
    }

    env_logger::builder()
        .filter(Some("actix_server::builder"), LevelFilter::Info)
        .init();

    // Configure database and migrate
    let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_e| {
        error!("DATABASE_URL environment variable not set");
        std::process::exit(1);
    });

    let pool = Arc::new(init_pool(database_url.as_str()));

    run_db_migrations(pool.clone())
        .await
        .err()
        .map(|e| format_err!("Database migration failed with error: {}", e));

    // Configure repository
    let repository = Arc::new(FootballerRepository::new(pool.clone()));

    // Start the web server
    HttpServer::new(move || {
        App::new()
            .data(repository.clone())
            .wrap(middleware::Logger::default())
            .service(footballer_rest_controller::footballer_create)
            .service(footballer_rest_controller::footballer_delete)
            .service(footballer_rest_controller::footballer_get)
            .service(footballer_rest_controller::footballer_search)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await?;

    Ok(())
}

pub fn init_pool(connection_string: &str) -> Pool<ConnectionManager<PgConnection>> {
    let connection_manager = ConnectionManager::<PgConnection>::new(connection_string);
    Pool::builder()
        .max_size(15)
        .min_idle(Some(5))
        .build(connection_manager)
        .expect("Failed to create db connection pool")
}
