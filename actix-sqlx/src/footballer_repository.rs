use sqlx::{Error, PgPool, Row};
use sqlx::postgres::PgRow;
use sqlx::query::query;

use async_trait::async_trait;

use crate::footballer::{Footballer, NewFootballer};

#[async_trait]
pub trait FootballerRepository {
    async fn create(&self, p_footballer: &NewFootballer) -> Result<Footballer, Error>;
    async fn find_by_position(&self, p_position: String) -> Result<Vec<Footballer>, Error>;
    async fn find_by_id(&self, p_id: i64) -> Result<Footballer, Error>;
    async fn find_all(&self) -> Result<Vec<Footballer>, Error>;
    async fn delete_by_id(&self, p_id: i64) -> Result<bool, Error>;
}

#[async_trait]
impl FootballerRepository for PgPool {
    async fn create(&self, p_footballer: &NewFootballer) -> Result<Footballer, Error> {
        let mut tx = self.begin().await.unwrap();

        let footballer = query(
            "insert into footballer (first_name, last_name, position) values ($1,$2,$3) returning id, first_name, last_name, position",
        )
        .bind(&p_footballer.first_name)
        .bind(&p_footballer.last_name)
        .bind(&p_footballer.position)
        .map(map_row)
        .fetch_one(&mut tx)
        .await?;

        tx.commit().await?;

        Ok(footballer)
    }

    async fn find_by_position(&self, p_position: String) -> Result<Vec<Footballer>, Error> {
        let footballers =
            query("select id, first_name, last_name, position from footballer where position = $1")
                .bind(p_position)
                .map(map_row)
                .fetch_all(self)
                .await?;

        Ok(footballers)
    }

    async fn find_by_id(&self, p_id: i64) -> Result<Footballer, Error> {
        let footballer =
            query("select id, first_name, last_name, position from footballer where id = $1")
                .bind(p_id)
                .map(map_row)
                .fetch_one(self)
                .await?;

        Ok(footballer)
    }

    async fn find_all(&self) -> Result<Vec<Footballer>, Error> {
        let footballers = query("select id, first_name, last_name, position from footballer")
            .map(map_row)
            .fetch_all(self)
            .await?;

        Ok(footballers)
    }

    async fn delete_by_id(&self, p_id: i64) -> Result<bool, Error> {
        let tx = self.begin().await.unwrap();

        let deleted = query("delete from footballer where id = $1")
            .bind(p_id)
            .execute(self)
            .await?;

        tx.commit().await?;

        Ok(deleted > 0)
    }
}

fn map_row(row: PgRow) -> Footballer {
    Footballer {
        id: row.get(0),
        first_name: row.get(1),
        last_name: row.get(2),
        position: row.get(3),
    }
}
