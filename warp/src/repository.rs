use crate::footballer::Footballer;
use anyhow::Context;
use futures::stream::TryStreamExt;
use serde::Deserialize;
use sqlx::{postgres::PgQueryAs, PgConnection, Pool};
use tracing::info;

#[derive(Deserialize, Debug)]
pub struct SearchFootballersOptions {
    position: Option<String>,
}

pub(crate) async fn search_footballers(
    pool: Pool<PgConnection>,
    options: SearchFootballersOptions,
) -> anyhow::Result<Vec<Footballer>> {
    info!("search_footballers ({:?})", options);
    let query = match options.position {
        None => "select * from footballer",
        Some(_) => "select * from footballer where position = $1",
    };
    sqlx::query_as::<_, Footballer>(&query)
        .bind(options.position)
        .fetch(&pool)
        .try_collect::<Vec<_>>()
        .await
        .context("search_footballers")
}

pub(crate) async fn get_footballer(
    pool: Pool<PgConnection>,
    id: i64,
) -> anyhow::Result<Option<Footballer>> {
    info!("get_footballer ({:?})", id);
    let query = "select * from footballer where id = $1";
    sqlx::query_as::<_, Footballer>(query)
        .bind(id)
        .fetch_optional(&pool)
        .await
        .context("get_footballer")
}

pub(crate) async fn create_footballer(
    pool: Pool<PgConnection>,
    create_request: crate::api::CreateFootballerRequest,
) -> anyhow::Result<Footballer> {
    info!("create_footballer ({:?})", create_request);
    let query =
        "insert into footballer(first_name, last_name, position) values ($1, $2, $3) returning *";
    sqlx::query_as::<_, Footballer>(query)
        .bind(create_request.first_name)
        .bind(create_request.last_name)
        .bind(create_request.position)
        .fetch_one(&pool)
        .await
        .context("create_footballer")
}

pub(crate) async fn delete_footballer(pool: Pool<PgConnection>, id: i64) -> anyhow::Result<u64> {
    info!("delete_footballer ({:?})", id);
    let query = "delete from footballer where id = $1";
    sqlx::query(query)
        .bind(id)
        .execute(&pool)
        .await
        .context("delete_footballer")
}
