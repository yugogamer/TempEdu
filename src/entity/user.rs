use serde::{Serialize, Deserialize};
use tokio_pg_mapper_derive::PostgresMapper;




#[derive(Debug, Clone, Serialize, Deserialize, PostgresMapper)]
#[pg_mapper(table = "accounts")]
pub struct User{
    pub id: i32,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub abreviate_name: String,
    pub mail: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PostgresMapper)]
#[pg_mapper(table = "accounts")]
pub struct UserInsertion{
    pub username: String,
    pub mdp: String,
    pub first_name: String,
    pub last_name: String,
    pub abreviate_name: String,
    pub mail: String,
}