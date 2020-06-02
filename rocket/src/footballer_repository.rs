use diesel::prelude::*;
use diesel::result::Error;

use crate::footballer::{Footballer, NewFootballer};
use crate::schema::footballer::dsl::*;

pub trait FootballerRepository {
    fn create(&self, p_footballer: &NewFootballer) -> Result<Footballer, Error>;
    fn find_by_position(&self, p_position: &String) -> Result<Vec<Footballer>, Error>;
    fn find_by_id(&self, p_id: i64) -> Result<Footballer, Error>;
    fn find_all(&self) -> Result<Vec<Footballer>, Error>;
    fn delete_by_id(&self, p_id: i64) -> Result<bool, Error>;
}

impl FootballerRepository
    for diesel::r2d2::PooledConnection<diesel::r2d2::ConnectionManager<diesel::PgConnection>>
{
    fn create(&self, p_footballer: &NewFootballer) -> Result<Footballer, Error> {
        diesel::insert_into(footballer)
            .values(p_footballer)
            .get_result::<Footballer>(self)
    }

    fn find_by_position(self: &Self, p_position: &String) -> Result<Vec<Footballer>, Error> {
        footballer
            .filter(position.eq(p_position))
            .load::<Footballer>(self)
    }

    fn find_by_id(self: &Self, p_id: i64) -> Result<Footballer, Error> {
        footballer.filter(id.eq(p_id)).first::<Footballer>(self)
    }

    fn find_all(self: &Self) -> Result<Vec<Footballer>, Error> {
        footballer.load::<Footballer>(self)
    }

    fn delete_by_id(self: &Self, p_id: i64) -> Result<bool, Error> {
        let count = diesel::delete(footballer.filter(id.eq(p_id))).execute(self)?;
        Ok(count > 0)
    }
}
