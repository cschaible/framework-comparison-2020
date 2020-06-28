use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow, Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Footballer {
    pub id: i64,
    #[serde(rename = "firstName")]
    pub first_name: Option<String>,
    #[serde(rename = "lastName")]
    pub last_name: Option<String>,
    pub position: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct NewFootballer {
    #[serde(rename = "firstName")]
    pub first_name: Option<String>,
    #[serde(rename = "lastName")]
    pub last_name: Option<String>,
    pub position: Option<String>,
}
