use diesel::{prelude::*, result::Error};

use crate::{
    footballer::{Footballer, NewFootballer},
    schema::footballer::dsl::*,
};

pub trait FootballerRepository {
    fn create(&self, p_footballer: &NewFootballer) -> Result<Footballer, Error>;
    fn find_by_position(&self, p_position: &str) -> Result<Vec<Footballer>, Error>;
    fn find_by_id(&self, p_id: i64) -> Result<Option<Footballer>, Error>;
    fn find_all(&self) -> Result<Vec<Footballer>, Error>;
    fn delete_by_id(&self, p_id: i64) -> Result<bool, Error>;
}

impl FootballerRepository for PgConnection {
    fn create(&self, p_footballer: &NewFootballer) -> Result<Footballer, Error> {
        Ok(self.transaction::<_, diesel::result::Error, _>(|| {
            Ok(diesel::insert_into(footballer)
                .values(p_footballer)
                .get_result::<Footballer>(self)?)
        })?)
    }

    fn find_by_position(self: &Self, p_position: &str) -> Result<Vec<Footballer>, Error> {
        Ok(read_only_transaction(self, || {
            footballer
                .filter(position.eq(p_position))
                .load::<Footballer>(self)
        })?)
    }

    fn find_by_id(self: &Self, p_id: i64) -> Result<Option<Footballer>, Error> {
        Ok(footballer
            .filter(id.eq(p_id))
            .first::<Footballer>(self)
            .optional()?)
    }

    fn find_all(self: &Self) -> Result<Vec<Footballer>, Error> {
        Ok(footballer.load::<Footballer>(self)?)
    }

    fn delete_by_id(self: &Self, p_id: i64) -> Result<bool, Error> {
        let count = self.transaction::<_, diesel::result::Error, _>(|| {
            Ok(diesel::delete(footballer.filter(id.eq(p_id))).execute(self)?)
        })?;
        Ok(count > 0)
    }
}

fn read_only_transaction<T, E, F>(conn: &PgConnection, f: F) -> core::result::Result<T, E>
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
