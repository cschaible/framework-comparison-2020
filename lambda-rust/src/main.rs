#![feature(option_result_contains)]
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use std::error::Error;

use anyhow::format_err;
use http::{Method, StatusCode};
use lambda_http::{IntoResponse, lambda, Request, RequestExt};
use lambda_runtime::{Context, error::HandlerError};
use log::info;
use postgres::{Client, NoTls};
use serde_json::from_str;

pub mod api;
pub mod footballer;
pub mod repository;

fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().ok();
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "debug")
    };

    env_logger::builder().init();

    lambda!(router);

    Ok(())
}

fn router(req: Request, _: Context) -> Result<impl IntoResponse, HandlerError> {
    let mut connection = get_db_connection().expect("no db connection");

    match *req.method() {
        Method::POST => api::create(req, &mut connection),
        Method::GET => {
            info!("Query Parameters: {:?}", req.query_string_parameters());
            info!("Path Parameters: {:?}", req.path_parameters());
            match req.path_parameters().get("id") {
                Some(id) => match from_str::<i64>(id) {
                    Ok(id) => api::find_by_id(id, &mut connection),
                    Err(_) => api::error_response(StatusCode::BAD_REQUEST, "Invalid id provided"),
                },
                _ => match req.query_string_parameters().get("position") {
                    Some(position) => api::find_by_position(position, &mut connection),
                    _ => api::find_all(&mut connection),
                },
            }
        }
        Method::DELETE => match req.path_parameters().get("id") {
            Some(id) => match from_str::<i64>(id) {
                Ok(id) => api::delete_by_id(id, &mut connection),
                Err(_) => api::error_response(StatusCode::BAD_REQUEST, "Invalid id provided"),
            },
            _ => api::error_response(StatusCode::NOT_FOUND, "Not found"),
        },
        _ => api::error_response(StatusCode::NOT_FOUND, "Not found"),
    }
}

fn get_db_connection() -> anyhow::Result<Client> {
    let database_url = std::env::var("DATABASE_URL")
        .map_err(|_| format_err!("DATABASE_URL environment variable not set"))?;

    Ok(Client::connect(database_url.as_str(), NoTls)?)
}
