use http::StatusCode;
use lambda_http::{Body, IntoResponse, Request, Response};
use lambda_runtime::error::HandlerError;
use log::warn;
use postgres::{Client, Error};

use crate::{footballer::NewFootballer, repository::FootballerRepository};

pub fn create(req: Request, connection: &mut Client) -> Result<Response<Body>, HandlerError> {
    match serde_json::from_slice::<NewFootballer>(req.body().as_ref()) {
        Ok(new_footballer) => match connection.create(&new_footballer) {
            Ok(footballer) => {
                let mut response = serde_json::json!(footballer).into_response();
                *response.status_mut() = StatusCode::CREATED;
                Ok(response)
            }
            Err(e) => handle_error(e),
        },
        Err(_) => error_response(StatusCode::BAD_REQUEST, "Invalid input"),
    }
}

pub fn find_all(connection: &mut Client) -> Result<Response<Body>, HandlerError> {
    match connection.find_all() {
        Ok(footballers) => {
            let mut response = serde_json::json!(footballers).into_response();
            *response.status_mut() = StatusCode::OK;
            Ok(response)
        }
        Err(e) => handle_error(e),
    }
}

pub fn find_by_position(
    position: &str,
    connection: &mut Client,
) -> Result<Response<Body>, HandlerError> {
    match connection.find_by_position(position) {
        Ok(footballers) => {
            let mut response = serde_json::json!(footballers).into_response();
            *response.status_mut() = StatusCode::OK;
            Ok(response)
        }
        Err(e) => handle_error(e),
    }
}

pub fn find_by_id(id: i64, connection: &mut Client) -> Result<Response<Body>, HandlerError> {
    match connection.find_by_id(id) {
        Ok(footballer) => {
            let mut response = serde_json::json!(footballer).into_response();
            *response.status_mut() = StatusCode::OK;
            Ok(response)
        }
        Err(e) => handle_error(e),
    }
}

pub fn delete_by_id(id: i64, connection: &mut Client) -> Result<Response<Body>, HandlerError> {
    match connection.delete_by_id(id) {
        Ok(deleted) => {
            let mut response = serde_json::json!(deleted).into_response();
            *response.status_mut() = StatusCode::NO_CONTENT;
            Ok(response)
        }
        Err(e) => handle_error(e),
    }
}

fn handle_error(err: Error) -> Result<Response<Body>, HandlerError> {
    warn!("Sql State: {:?}", err.code());
    if err.code() != None {
        error_response(StatusCode::INTERNAL_SERVER_ERROR, err.to_string().as_str())
    } else {
        error_response(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
    }
}

pub fn error_response(
    status_code: StatusCode,
    message: &str,
) -> Result<Response<Body>, HandlerError> {
    Ok(Response::builder()
        .status(status_code)
        .body(message.into())
        .expect("err creating response"))
}
