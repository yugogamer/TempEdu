use serde::{Serialize, Deserialize};
use tokio_pg_mapper_derive::PostgresMapper;

#[derive(Debug, Serialize, Deserialize, PostgresMapper)]
#[pg_mapper(table = "groupes")]
pub struct Groupe{
    pub id: i32,
    pub name: String,
    pub protected: bool
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InsertGroupe{
    pub name: String,
    pub protected: Option<bool>
}