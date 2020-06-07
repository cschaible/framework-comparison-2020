use actix_http::ResponseBuilder;
use actix_web::{delete, Error, get, HttpRequest, HttpResponse, post};
use actix_web::web::{Data, Json, Query};
use anyhow::Result;
use serde::Deserialize;
use sqlx::PgPool;

use crate::footballer::NewFootballer;
use crate::footballer_repository::FootballerRepository;

#[derive(Deserialize)]
pub struct SearchParameters {
    position: Option<String>,
}

#[get("/footballers")]
pub async fn footballer_search(
    Query(params): Query<SearchParameters>,
    db: Data<PgPool>,
) -> Result<HttpResponse, Error> {
    Ok(match params.position {
        Some(pos) => db.find_by_position(pos),
        None => db.find_all(),
    }
    .await
    .map(|footballers| HttpResponse::Ok().json(footballers))
    .map_err(handle_error)?)
}

#[get("/footballers/{id}")]
pub async fn footballer_get(req: HttpRequest, db: Data<PgPool>) -> Result<HttpResponse, Error> {
    let id: i64 = req.match_info().query("id").parse().unwrap();
    Ok(db
        .find_by_id(id)
        .await
        .map(|footballer| HttpResponse::Ok().json(footballer))
        .map_err(handle_error)?)
}

#[post("/footballers")]
pub async fn footballer_create(
    footballer: Json<NewFootballer>,
    db: Data<PgPool>,
) -> Result<HttpResponse, Error> {
    Ok(db
        .create(&footballer.into_inner())
        .await
        .map(|footballer| HttpResponse::Ok().json(footballer))
        .map_err(handle_error)?)
}

#[delete("/footballers/{id}")]
pub async fn footballer_delete(req: HttpRequest, db: Data<PgPool>) -> Result<HttpResponse, Error> {
    let id: i64 = req.match_info().query("id").parse().unwrap();
    Ok(db
        .delete_by_id(id)
        .await
        .map(|_| HttpResponse::NoContent().finish())
        .map_err(handle_error)?)
}

fn handle_error(e: sqlx::Error) -> ResponseBuilder {
    match e {
        sqlx::Error::RowNotFound => HttpResponse::NotFound(),
        _ => HttpResponse::InternalServerError(),
    }
}
