use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use tokio_pg_mapper_derive::PostgresMapper;


#[derive(Debug, Clone, Serialize, Deserialize, PostgresMapper)]
#[pg_mapper(table = "weeks")]
pub struct Week{
    pub id : i32,
    pub week: i32,
    pub year: i32,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub visible: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeekInsertion{
    pub iso_string : String,
    pub visible: bool,
}