use serde::Serialize;

#[derive(sqlx::FromRow, Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Serialize)]
pub(crate) struct Footballer {
    id: i64,
    first_name: String,
    last_name: String,
    position: String,
}
