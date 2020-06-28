use std::convert::Infallible;

use serde::Deserialize;
use sqlx::{PgConnection, Pool};
use tracing::{error, info};
use warp::{
    body,
    http::StatusCode,
    reject::{self, Reject},
    reply, Filter, Rejection,
};

pub fn search_footballers(
    pool: Pool<PgConnection>,
) -> impl Filter<Extract = impl warp::Reply, Error = Rejection> + Clone {
    warp::get()
        .and(warp::path("footballers"))
        .and(warp::path::end())
        .and(warp::query())
        .and(with_db(pool))
        .and_then(|pos, pool| async {
            match crate::repository::search_footballers(pool, pos).await {
                Ok(footballers) => {
                    info!("Found footballers: {:?}", footballers);
                    Ok(reply::json(&footballers))
                }
                Err(e) => {
                    error!("Error: {:?}", e);
                    Err(reject::custom(InternalServerError))
                }
            }
        })
}

pub fn get_footballer(
    pool: Pool<PgConnection>,
) -> impl Filter<Extract = impl warp::Reply, Error = Rejection> + Clone {
    warp::get()
        .and(warp::path("footballers"))
        .and(warp::path::param())
        .and(with_db(pool))
        .and_then(|id, pool| async move {
            match crate::repository::get_footballer(pool, id).await {
                Ok(footballer) => {
                    info!("Found footballer {:?}", footballer);
                    let status = match footballer {
                        Some(_) => StatusCode::OK,
                        None => StatusCode::NOT_FOUND,
                    };
                    Ok(reply::with_status(reply::json(&footballer), status))
                }
                Err(e) => {
                    error!("Error: {:?}", e);
                    Err(reject::custom(InternalServerError))
                }
            }
        })
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct CreateFootballerRequest {
    pub(crate) first_name: Option<String>,
    pub(crate) last_name: Option<String>,
    pub(crate) position: Option<String>,
}

pub fn create_footballer(
    pool: Pool<PgConnection>,
) -> impl Filter<Extract = impl warp::Reply, Error = Rejection> + Clone {
    warp::post()
        .and(warp::path("footballers"))
        .and(body::json())
        .and(with_db(pool))
        .and_then(|create_request, pool| async move {
            match crate::repository::create_footballer(pool, create_request).await {
                Ok(created) => {
                    info!("Created footballer {:?}", created);
                    Ok(reply::with_status(
                        reply::json(&created),
                        StatusCode::CREATED,
                    ))
                }
                Err(e) => {
                    error!("Error: {:?}", e);
                    Err(reject::custom(InternalServerError))
                }
            }
        })
}

pub fn delete_footballer(
    pool: Pool<PgConnection>,
) -> impl Filter<Extract = impl warp::Reply, Error = Rejection> + Clone {
    warp::delete()
        .and(warp::path("footballers"))
        .and(warp::path::param())
        .and(with_db(pool))
        .and_then(|id, pool| async move {
            match crate::repository::delete_footballer(pool, id).await {
                Ok(deleted) => {
                    info!("Deleted footballer {:?}", deleted);
                    Ok(reply::with_status("", StatusCode::NO_CONTENT))
                }
                Err(e) => {
                    error!("Error: {:?}", e);
                    Err(reject::custom(InternalServerError))
                }
            }
        })
}

fn with_db(
    pool: Pool<PgConnection>,
) -> impl Filter<Extract = (Pool<PgConnection>,), Error = Infallible> + Clone {
    warp::any().map(move || pool.clone())
}

#[derive(Debug)]
struct InternalServerError;

impl Reject for InternalServerError {}
