#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

use std::collections::HashMap;

use anyhow::format_err;
use log::{error, info};
use rocket::{
    catchers,
    config::{Config, Environment, Limits, Value},
    fairing::AdHoc,
    routes, Rocket,
};
use rocket_contrib::database;

pub mod api;
pub mod footballer;
pub mod footballer_repository;
pub mod schema;

embed_migrations!();

#[database("pg")]
pub struct PgDatabase(diesel::PgConnection);

fn run_db_migrations(rocket: Rocket) -> Result<Rocket, Rocket> {
    let connection = PgDatabase::get_one(&rocket).expect("database connection");
    info!("Migrate Database");
    match embedded_migrations::run(&*connection) {
        Ok(()) => Ok(rocket),
        Err(e) => {
            error!("Failed to run database migrations: {:?}", e);
            Err(rocket)
        }
    }
}

fn main() {
    if let Err(e) = run() {
        error!("{:?}", e);
        std::process::exit(1);
    }
}

fn run() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "warn")
    };

    env_logger::builder().init();

    let database_url = std::env::var("DATABASE_URL")
        .map_err(|_| format_err!("DATABASE_URL environment variable not set"))?;

    let mut database_config = HashMap::new();
    let mut databases = HashMap::new();

    database_config.insert("url", Value::from(database_url));
    database_config.insert("pool_size", Value::from(15));
    databases.insert("pg", Value::from(database_config));

    let config = Config::build(Environment::Staging)
        .keep_alive(0)
        .extra("databases", databases)
        .port(8080)
        .limits(Limits::new().limit("json", 1 * 1024 * 1024))
        .finalize()?;

    let launch_err = rocket::custom(config)
        .attach(PgDatabase::fairing())
        .attach(AdHoc::on_attach("Migrate Database", run_db_migrations))
        .mount(
            "/",
            routes![
                api::footballers_search,
                api::footballer_get,
                api::footballer_create,
                api::footballer_delete
            ],
        )
        .register(catchers![api::not_found])
        .launch();
    Err(launch_err.into())
}
