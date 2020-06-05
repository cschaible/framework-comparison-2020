use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Queryable)]
pub struct Footballer {
    pub id: i64,
    #[serde(rename = "firstName")]
    pub first_name: Option<String>,
    #[serde(rename = "lastName")]
    pub last_name: Option<String>,
    pub position: Option<String>,
}

use crate::schema::footballer;

#[derive(Clone, Debug, Deserialize, Insertable)]
#[table_name = "footballer"]
pub struct NewFootballer {
    #[serde(rename = "firstName")]
    pub first_name: Option<String>,
    #[serde(rename = "lastName")]
    pub last_name: Option<String>,
    pub position: Option<String>,
}
