use actix_web::{HttpResponse, error};
use deadpool_postgres::Client;
use thiserror::Error;
use tokio_pg_mapper::FromTokioPostgresRow;

use crate::entity::groupes::{InsertGroupe, Groupe};



async fn create_groupe(conn: &Client, groupe : InsertGroupe) -> Result<(), GroupeError>{
    let query = r#"
        INSERT INTO groupes (name, protected)
        VALUES ($1, $2)
        RETURNING id
    "#;

    let row = conn.query_one(query, &[&groupe.name, &groupe.protected]).await;
    match row {
        Ok(_) => Ok(()),
        Err(err) => {
            println!("{:?}", err);
            Err(GroupeError::GroupeNotValid)
        }
    }
}

async fn get_groupes_of_user(conn: &Client, id_user : i32) -> Result<Vec<Groupe>, GroupeError>{
    let query = r#"
        SELECT id, name, protected
        FROM groupes G
        LEFT OUTER JOIN accountsToGroupes AG ON AG.id_groupe = G.id
        WHERE AG.id_user = $1
    "#;

    let rows = conn.query(query, &[&id_user]).await?;

    let mut list = Vec::new();

    for row in rows{
        let groupe = Groupe::from_row(row)?;
        list.push(groupe);
    }

    Ok(list)
}


#[derive(Debug, Error)]
pub enum GroupeError{
    #[error("crenaux not found")]
    GroupeNotFound,
    #[error("crenaux not valid")]
    GroupeNotValid,
    #[error("api : mapper error")]
    MapperError(#[from] tokio_pg_mapper::Error),
    #[error("api : db error")]
    DbError(#[from] tokio_postgres::Error),
    #[error("groupe already exit")]
    GroupeAlreadyExists,
}

impl error::ResponseError for GroupeError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GroupeError::GroupeNotFound => HttpResponse::NotFound().body(self.to_string()),
            GroupeError::GroupeNotValid => HttpResponse::BadRequest().body(self.to_string()),
            GroupeError::MapperError(_) => HttpResponse::InternalServerError().body(self.to_string()),
            GroupeError::GroupeAlreadyExists => HttpResponse::Conflict().body(self.to_string()),
            GroupeError::DbError(_) => HttpResponse::InternalServerError().body(self.to_string()),
        }
    }
}