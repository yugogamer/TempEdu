use tokio_pg_mapper::FromTokioPostgresRow;
use tokio_postgres::Client;
use std::{error::Error};

use crate::entity::user::{User, UserInsertion};


pub async fn get_user(conn : &Client, id : i32) -> Result<User, Box<dyn Error>>{
    let row = conn.query_one("SELECT * FROM accounts WHERE id = $1", &[&id]).await?;

    let user = User::from_row(row)?;

    Ok(user)
}

pub async fn get_user_by_session(conn : &Client, uuid : uuid::Uuid) -> Result<User, Box<dyn Error>>{
    let row = conn.query_one("SELECT A.id, A.username, A.first_name, A.last_name, A.abreviate_name, A.mail, S.expiration_date FROM accounts as A, session as S WHERE S.id = $1", &[&uuid]).await?;

    let user = User::from_row(row)?;

    Ok(user)
}

pub async fn add_user(conn : &Client, user : &UserInsertion) -> Result<User, Box<dyn Error>>{
    let row = conn.query("INSERT INTO accounts (username, password, first_name, last_name, abreviate_name, mail) VALUES ($1, crypt($2, gen_salt('bf')), $3, $4, $5, $6) RETURNING id, username, first_name, last_name, abreviate_name, mail", &[&user.username, &user.mdp, &user.first_name, &user.last_name, &user.abreviate_name, &user.mail]).await?;
    
    for row in row {
        let user = User::from_row(row)?;
        return Ok(user);
    }

    Err("user not inserted".into())
}