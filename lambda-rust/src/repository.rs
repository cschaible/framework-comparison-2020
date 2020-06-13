use postgres::{Client, Error, Row};

use crate::footballer::{Footballer, NewFootballer};

pub trait FootballerRepository {
    fn create(&mut self, p_footballer: &NewFootballer) -> Result<Footballer, Error>;
    fn find_by_position(&mut self, p_position: &str) -> Result<Vec<Footballer>, Error>;
    fn find_by_id(&mut self, p_id: i64) -> Result<Footballer, Error>;
    fn find_all(&mut self) -> Result<Vec<Footballer>, Error>;
    fn delete_by_id(&mut self, p_id: i64) -> Result<bool, Error>;
}

impl FootballerRepository for Client {
    fn create(&mut self, p_footballer: &NewFootballer) -> Result<Footballer, Error> {
        let row = self.query_one("insert into footballer (first_name, last_name, position) values ($1,$2,$3) returning id, first_name, last_name, position",
        &[&p_footballer.first_name, &p_footballer.last_name, &p_footballer.position])?;
        Ok(Footballer {
            id: row.get("id"),
            first_name: row.get("first_name"),
            last_name: row.get("last_name"),
            position: row.get("position"),
        })
    }

    fn find_by_position(&mut self, p_position: &str) -> Result<Vec<Footballer>, Error> {
        let rows = self.query(
            "select id, first_name, last_name, position from footballer where position = $1",
            &[&p_position],
        )?;
        Ok(map_rows(rows))
    }

    fn find_by_id(&mut self, p_id: i64) -> Result<Footballer, Error> {
        let row = self.query_one(
            "select id, first_name, last_name, position from footballer where id = $1",
            &[&p_id],
        )?;
        Ok(map_row(row))
    }

    fn find_all(&mut self) -> Result<Vec<Footballer>, Error> {
        let rows = self.query(
            "select id, first_name, last_name, position from footballer",
            &[],
        )?;
        Ok(map_rows(rows))
    }

    fn delete_by_id(&mut self, p_id: i64) -> Result<bool, Error> {
        let rows_deleted = self.execute("delete from footballer where id = $1", &[&p_id])?;
        Ok(rows_deleted > 0)
    }
}

fn map_rows(rows: Vec<Row>) -> Vec<Footballer> {
    let mut footballers = Vec::<Footballer>::new();
    for row in rows {
        footballers.push(map_row(row));
    }
    footballers
}

fn map_row(row: Row) -> Footballer {
    Footballer {
        id: row.get(0),
        first_name: row.get(1),
        last_name: row.get(2),
        position: row.get(3),
    }
}
