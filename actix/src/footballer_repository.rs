use std::sync::Arc;

use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use diesel::PgConnection;

use crate::footballer::Footballer;
use crate::footballer::NewFootballer;

pub struct FootballerRepository {
    m_pool: Arc<Pool<ConnectionManager<PgConnection>>>,
}

impl FootballerRepository {
    pub fn new(pool: Arc<Pool<ConnectionManager<PgConnection>>>) -> FootballerRepository {
        FootballerRepository { m_pool: pool }
    }

    /// Save a footballer in the database
    pub fn create(&self, v_footballer: NewFootballer) -> Result<Footballer, Error> {
        use crate::schema::footballer::dsl::*;

        // Get connection from pool
        let conn = &*self.m_pool.get()?;

        // Start transaction and save data
        Ok(conn.transaction::<_, diesel::result::Error, _>(|| {
            Ok(diesel::insert_into(footballer)
                .values(&v_footballer)
                .get_result::<Footballer>(conn)?)
        })?)
    }

    /// Find a footballer by its position
    pub fn find_by_position(&self, footballer_position: &str) -> Result<Vec<Footballer>, Error> {
        use crate::schema::footballer::dsl::*;

        // Get connection from pool
        let conn = &*self.m_pool.get()?;

        // Start transaction and load data
        Ok(self.read_only_transaction(conn, || {
            footballer
                .filter(position.eq(footballer_position))
                .load::<Footballer>(conn)
        })?)
    }

    /// Find a footballer by its id
    pub fn find_by_id(&self, footballer_id: i64) -> Result<Option<Footballer>, Error> {
        use crate::schema::footballer::dsl::*;

        // Get connection from pool
        let conn = &*self.m_pool.get()?;

        // Load data
        Ok(footballer
            .filter(id.eq(footballer_id))
            .first::<Footballer>(conn)
            .optional()?)
    }

    /// Find all footballers
    pub fn find_all(&self) -> Result<Vec<Footballer>, Error> {
        use crate::schema::footballer::dsl::*;

        // Get connection from pool
        let conn = &*self.m_pool.get()?;

        // Load data
        Ok(footballer.load::<Footballer>(conn)?)
    }

    /// Delete a footballer by its id
    pub fn delete_by_id(&self, footballer_id: i64) -> Result<bool, Error> {
        use crate::schema::footballer::dsl::*;

        // Get connection from pool
        let conn = &*self.m_pool.get()?;

        // Start transaction and delete data
        let count = conn.transaction::<_, diesel::result::Error, _>(|| {
            Ok(diesel::delete(footballer.filter(id.eq(footballer_id))).execute(conn)?)
        })?;
        Ok(count > 0)
    }

    fn read_only_transaction<T, E, F>(
        &self,
        conn: &PgConnection,
        f: F,
    ) -> core::result::Result<T, E>
    where
        F: FnOnce() -> Result<T, E>,
        E: From<diesel::result::Error>,
    {
        match conn
            .build_transaction()
            .read_only()
            .read_committed()
            .run::<_, diesel::result::Error, _>(|| Ok(f()))
        {
            Ok(result) => result,
            Err(e) => Err(e.into()),
        }
    }
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
