use std::str::FromStr;

use actix_web::{dev::ServiceRequest, web::{self, Data}};
use deadpool_postgres::{Pool, PoolError};
use tokio_pg_mapper::FromTokioPostgresRow;
use tokio_postgres::{Client};
use thiserror::Error;
use uuid::Uuid;



pub async fn set_role(conn: &Client, id_user: i32, id_role: i32) -> bool {
    let row = conn.query(
        "INSERT INTO roletousers(id_user, id_role) VALUES ($1, $2) 
        ON CONFLICT (id_user, id_role) DO UPDATE SET id_user = $1, id_role = $2"
        , &[&id_role, &id_user]).await;

    match row {
        Ok(_) => true,
        Err(_) => false
    }
}