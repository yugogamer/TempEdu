use chrono::Utc;
use serde::{Serialize, Deserialize};
use tokio_pg_mapper_derive::PostgresMapper;


#[derive(Debug, Serialize, Deserialize, PostgresMapper)]
#[pg_mapper(table = "creneaux")]
pub struct Crenau{
    pub id: i32,
    pub id_week: i32,
    pub id_matiere : Option<i32>,
    pub start_time: chrono::DateTime<Utc>,
    pub end_time: chrono::DateTime<Utc>,
    pub name: Option<String>,
    pub description: Option<String>,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct InsertCrenaux{
    pub id_week: i32,
    pub id_matiere : Option<i32>,
    pub start_time: chrono::DateTime<Utc>,
    pub end_time: chrono::DateTime<Utc>,
    pub name: Option<String>,
    pub description: Option<String>,
}