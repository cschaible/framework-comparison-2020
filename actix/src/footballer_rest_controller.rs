use std::sync::Arc;

use actix_web::web::{Data, Json, Query};
use actix_web::{delete, get, post, web, Error, HttpRequest, HttpResponse};
use serde::Deserialize;

use crate::footballer::NewFootballer;
use crate::footballer_repository::FootballerRepository;

#[derive(Deserialize)]
pub struct SearchParameters {
    position: Option<String>,
}

#[get("/footballers")]
pub async fn footballer_search(
    Query(params): Query<SearchParameters>,
    footballer_repository: Data<Arc<FootballerRepository>>,
) -> Result<HttpResponse, Error> {
    Ok(web::block(move || match params.position {
        Some(pos) => footballer_repository.find_by_position(pos.as_str()),
        None => footballer_repository.find_all(),
    })
    .await
    .map(|footballers| HttpResponse::Ok().json(footballers))
    .map_err(|_| HttpResponse::InternalServerError())?)
}

#[get("/footballers/{id}")]
pub async fn footballer_get(
    req: HttpRequest,
    footballer_repository: Data<Arc<FootballerRepository>>,
) -> Result<HttpResponse, Error> {
    let id: i64 = req.match_info().query("id").parse().unwrap();
    Ok(web::block(move || footballer_repository.find_by_id(id))
        .await
        .map(|footballer| HttpResponse::Ok().json(footballer))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

#[post("/footballers")]
pub async fn footballer_create(
    footballer: Json<NewFootballer>,
    footballer_repository: Data<Arc<FootballerRepository>>,
) -> Result<HttpResponse, Error> {
    Ok(
        web::block(move || footballer_repository.create(footballer.into_inner()))
            .await
            .map(|footballer| HttpResponse::Ok().json(footballer))
            .map_err(|_| HttpResponse::InternalServerError())?,
    )
}

#[delete("/footballers/{id}")]
pub async fn footballer_delete(
    req: HttpRequest,
    footballer_repository: Data<Arc<FootballerRepository>>,
) -> Result<HttpResponse, Error> {
    let id: i64 = req.match_info().query("id").parse().unwrap();
    Ok(web::block(move || footballer_repository.delete_by_id(id))
        .await
        .map(|_| HttpResponse::NoContent().finish())
        .map_err(|_| HttpResponse::InternalServerError().finish())?)
}
