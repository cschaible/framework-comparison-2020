use rocket::{
    catch, delete, get,
    http::Status,
    post,
    response::{content, status::Created, Responder},
    Request,
};
use rocket_contrib::json::Json;

use crate::{
    footballer::{Footballer, NewFootballer},
    footballer_repository::FootballerRepository,
    PgDatabase,
};

#[get("/footballers?<position>")]
pub fn footballers_search(
    connection: PgDatabase,
    position: Option<String>,
) -> Result<content::Json<Json<Vec<Footballer>>>, Error> {
    let footballers: Result<Vec<Footballer>, diesel::result::Error> = match position {
        Some(pos) => connection.0.find_by_position(&pos),
        None => connection.0.find_all(),
    };
    match footballers {
        Ok(footballers) => Ok(content::Json(Json(footballers))),
        Err(e) => Err(e.into()),
    }
}

#[get("/footballers/<id>")]
pub fn footballer_get(connection: PgDatabase, id: i64) -> Result<SingleFootballerResponse, Error> {
    match connection.0.find_by_id(id) {
        Ok(footballer) => match footballer {
            Some(f) => Ok(SingleFootballerResponse::Content(content::Json(Json(f)))),
            None => Ok(SingleFootballerResponse::ResponseStatus(Status::NotFound)),
        },
        Err(e) => Err(e.into()),
    }
}

#[post("/footballers", data = "<footballer>"/*, format = "json"*/)]
pub fn footballer_create(
    connection: PgDatabase,
    footballer: Json<NewFootballer>,
) -> Result<Created<content::Json<Json<Footballer>>>, Error> {
    match connection.0.create(&footballer.0) {
        Ok(footballer) => Ok(Created(
            "/footballers".to_string(),
            Some(content::Json(Json(footballer))),
        )),
        Err(e) => Err(e.into()),
    }
}

#[delete("/footballers/<id>")]
pub fn footballer_delete(connection: PgDatabase, id: i64) -> Status {
    match connection.0.delete_by_id(id) {
        Ok(_) => Status::NoContent,
        Err(_) => Status::BadRequest,
    }
}

#[catch(404)]
pub fn not_found(_: &Request) -> content::Json<String> {
    content::Json("{}".to_string())
}

#[derive(Debug)]
pub enum Error {
    DieselError(diesel::result::Error),
    NoDbConnectionError(r2d2::Error),
}

impl From<diesel::result::Error> for Error {
    fn from(e: diesel::result::Error) -> Self {
        Self::DieselError(e)
    }
}

impl From<r2d2::Error> for Error {
    fn from(e: r2d2::Error) -> Self {
        Self::NoDbConnectionError(e)
    }
}

#[derive(Debug, Responder)]
pub enum SingleFootballerResponse {
    ResponseStatus(Status),
    Content(content::Json<Json<Footballer>>),
}
