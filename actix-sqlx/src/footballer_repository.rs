use sqlx::{Error, PgPool, postgres::PgQueryAs, query::query, query_as};

use async_trait::async_trait;

use crate::footballer::{Footballer, NewFootballer};

#[async_trait]
pub trait FootballerRepository {
    async fn create(&self, p_footballer: &NewFootballer) -> Result<Footballer, Error>;
    async fn find_by_position(&self, p_position: String) -> Result<Vec<Footballer>, Error>;
    async fn find_by_id(&self, p_id: i64) -> Result<Option<Footballer>, Error>;
    async fn find_all(&self) -> Result<Vec<Footballer>, Error>;
    async fn delete_by_id(&self, p_id: i64) -> Result<bool, Error>;
}

#[async_trait]
impl FootballerRepository for PgPool {
    async fn create(&self, p_footballer: &NewFootballer) -> Result<Footballer, Error> {
        let mut tx = self.begin().await?;

        let query = "insert into footballer (first_name, last_name, position) values ($1,$2,$3) returning *";
        let footballer = query_as::<_, Footballer>(query)
            .bind(&p_footballer.first_name)
            .bind(&p_footballer.last_name)
            .bind(&p_footballer.position)
            .fetch_one(&mut tx)
            .await?;

        tx.commit().await?;

        Ok(footballer)
    }

    async fn find_by_position(&self, p_position: String) -> Result<Vec<Footballer>, Error> {
        let query =
            "select id, first_name, last_name, position from footballer where position = $1";
        let footballers = query_as::<_, Footballer>(query)
            .bind(p_position)
            .fetch_all(self)
            .await?;

        Ok(footballers)
    }

    async fn find_by_id(&self, p_id: i64) -> Result<Option<Footballer>, Error> {
        let query = "select id, first_name, last_name, position from footballer where id = $1";
        let footballer = query_as::<_, Footballer>(query)
            .bind(p_id)
            .fetch_optional(self)
            .await?;

        Ok(footballer)
    }

    async fn find_all(&self) -> Result<Vec<Footballer>, Error> {
        let query = "select id, first_name, last_name, position from footballer";
        let footballers = query_as::<_, Footballer>(query).fetch_all(self).await?;

        Ok(footballers)
    }

    async fn delete_by_id(&self, p_id: i64) -> Result<bool, Error> {
        let mut tx = self.begin().await?;

        let deleted = query("delete from footballer where id = $1")
            .bind(p_id)
            .execute(&mut tx)
            .await?;

        tx.commit().await?;

        Ok(deleted > 0)
    }
}
