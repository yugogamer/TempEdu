use tokio_pg_mapper::FromTokioPostgresRow;
use tokio_postgres::Client;
use std::error::Error;

use crate::entity::user::{User, UserInsertion};


pub async fn get_user(conn : &Client, id : i32) -> Result<User, Box<dyn Error>>{
    let row = conn.query_one("SELECT * FROM accounts WHERE id = $1", &[&id]).await?;

    let user = User::from_row(row)?;

    return Ok(user);
}

pub async fn add_user(conn : &Client, user : &UserInsertion) -> Result<User, Box<dyn Error>>{
    let row = conn.query("INSERT INTO accounts (username, first_name, last_name, abreviate_name, mail) VALUES ($1, $2, $3, $4, $5) RETURNING *", &[&user.username, &user.first_name, &user.last_name, &user.abreviate_name, &user.mail]).await?;
    
    for row in row {
        let user = User::from_row(row)?;
        return Ok(user);
    }

    return Err("user not inserted".into());
}