use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;

#[derive(Debug, Serialize, Deserialize, PostgresMapper)]
#[pg_mapper(table = "role")]
pub struct Role {
    pub id : i32,
    pub name : String,
    pub description : String,
}

#[derive(Debug, Serialize, Deserialize, PostgresMapper)]
#[pg_mapper(table = "roleToUsers")]
pub struct RoleToUser{
    pub id_role : i32,
    pub id_user : i32,
}

#[derive(Debug, Serialize, Deserialize, PostgresMapper)]
#[pg_mapper(table = "permission")]
pub struct Permission{
    pub id : i32,
    pub name : String,
    pub description : String,
}