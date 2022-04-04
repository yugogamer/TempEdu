use tokio_pg_mapper::FromTokioPostgresRow;
use tokio_postgres::Client;
use std::{error::Error};
use thiserror::Error;

use crate::entity::{user::{User, UserInsertion}};

#[derive(Debug, Error)]
pub enum UserError{
    #[error("Error api : mapper")]
    MapperError(#[from] tokio_pg_mapper::Error),
    #[error("Error api : database : `{0}`")]
    DbError(#[from] tokio_postgres::Error),
    #[error("Fake UUID")]
    UuidError(#[from] uuid::Error)
}


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

pub async fn get_permission_list(conn : &Client , id_user : i32) -> Result<Vec<String>,UserError>{
    let row = conn.query("
    SELECT P.name FROM roleToPermissions RP,  permissions P, roletousers RU, roles R 
    WHERE RU.id_user = $1 AND 
    RU.id_role = R.id AND 
    RP.id_role = RU.id_role AND
    P.id = RP.id_permission;", &[&id_user]).await?;

    let mut permissions_list : Vec<String> = Vec::new();

    for row in row {
        permissions_list.push(row.try_get("name")?);
    }

    Ok(permissions_list)
}