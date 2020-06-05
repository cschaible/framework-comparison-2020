use rocket_contrib::json::Json;

use rocket::http::Status;

use crate::{
    footballer::{Footballer, NewFootballer},
    footballer_repository::FootballerRepository,
    PgDatabase,
};
use diesel::result::Error;
use rocket::response::content;

#[get("/footballers?<position>")]
pub fn footballers_search(
    connection: PgDatabase,
    position: Option<String>,
) -> Result<content::Json<Json<Vec<Footballer>>>, Error> {
    let footballers: Result<Vec<Footballer>, Error> = match position {
        Some(pos) => connection.0.find_by_position(&pos),
        None => connection.0.find_all(),
    };
    match footballers {
        Ok(footballers) => Ok(content::Json(Json(footballers))),
        Err(e) => Err(e.into()),
    }
}

#[get("/footballers/<id>")]
pub fn footballer_get(
    connection: PgDatabase,
    id: i64,
) -> Result<content::Json<Json<Footballer>>, Error> {
    match connection.0.find_by_id(id) {
        Ok(footballer) => Ok(content::Json(Json(footballer))),
        Err(e) => Err(e.into()),
    }
}

#[post("/footballers", data = "<footballer>"/*, format = "json"*/)]
pub fn footballer_create(
    connection: PgDatabase,
    footballer: Json<NewFootballer>,
) -> Result<content::Json<Json<Footballer>>, Error> {
    match connection.0.create(&footballer.0) {
        Ok(footballer) => Ok(content::Json(Json(footballer))),
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
