use actix_web::{error, HttpResponse};
use thiserror::Error;
use tokio_pg_mapper::FromTokioPostgresRow;
use tokio_postgres::Client;

use crate::entity::crenaux::Crenau;



pub async fn get_personal_creneaux_of_user(conn: &Client , id : i32, week : i32) -> Result<Vec<Crenau>, CrenauxError> {
    let rows = conn.query("
    SELECT A.id, A.id_week, A.id_matiere, A.start_time, A.end_time, A.name.description, A.name FROM crenaux C, accountsToCreneaux AC  
    WHERE
    A.id_week = $1 AND 
    AC.id_user = $2"
    , &[&week ,&id]).await;
    if let Err(_err) = rows {
        return Err(CrenauxError::CreneauNotFound);
    }
    let crenaux = rows.unwrap();
    let mut crenaux_list : Vec<Crenau> = Vec::new();
    for crenaux in crenaux {
        let crenaux = Crenau::from_row(crenaux)?;
        crenaux_list.push(crenaux);
    }
    Ok(crenaux_list)
}


pub async fn get_creneaux_of_groupe(conn: &Client , id : i32, week : i32) -> Result<Vec<Crenau>, CrenauxError> {
    let rows = conn.query("
    SELECT A.id, A.id_week, A.id_matiere, A.start_time, A.end_time, A.name.description, A.name
    FROM crenaux C, groupesToCreneaux GC
    WHERE
    A.id_week = $1 AND 
    GC.id_groupe = $2 AND
    C.id = GC.id_creneau"
    , &[&week ,&id]).await;
    if let Err(_err) = rows {
        return Err(CrenauxError::CreneauNotFound);
    }
    let crenaux = rows.unwrap();
    let mut crenaux_list : Vec<Crenau> = Vec::new();
    for crenaux in crenaux {
        let crenaux = Crenau::from_row(crenaux)?;
        crenaux_list.push(crenaux);
    }
    Ok(crenaux_list)
}

pub async fn get_creneaux_of_user_with_groupe(conn: &Client , id : i32, week : i32) -> Result<Vec<Crenau>, CrenauxError> {
    let rows = conn.query("
    SELECT A.id, A.id_week, A.id_matiere, A.start_time, A.end_time, A.name.description, A.name
    FROM crenaux C, groupesToCreneaux GC, accountsToCreneaux AC
    WHERE
    A.id_week = $1 AND 
    GC.id_groupe IN (
        SELECT id_groupe FROM groupesToUsers WHERE id_user = $2
    ) AND 
    C.id = GC.id_creneau AND 
    AC.id_creneau = C.id AND AC.id_user = $2"
    , &[&week ,&id]).await;
    if let Err(_err) = rows {
        return Err(CrenauxError::CreneauNotFound);
    }
    let crenaux = rows.unwrap();
    let mut crenaux_list : Vec<Crenau> = Vec::new();
    for crenaux in crenaux {
        let crenaux = Crenau::from_row(crenaux)?;
        crenaux_list.push(crenaux);
    }
    Ok(crenaux_list)
}

pub async fn create_creneaux(conn: &Client , creneau : Crenau) -> Result<(), CrenauxError>{
    let row = conn.query("
    INSERT INTO creneaux (
        id_week,
        id_matiere,
        start_time,
        end_time,
        name,
        description
      )
    VALUES (
        $1,
        $2,
        $3,
        $4,
        $5,
        $6
      )", &[&creneau.id_week, &creneau.id_matiere, &creneau.start_time, &creneau.end_time, &creneau.name, &creneau.description]).await;
    if let Err(_err) = row {
        return Err(CrenauxError::CreneauNotValid);
    }
    Ok(())
}








#[derive(Debug, Error)]
pub enum CrenauxError{
    #[error("crenaux not found")]
    CreneauNotFound,
    #[error("crenaux not valid")]
    CreneauNotValid,
    #[error("api : mapper error")]
    MapperError(#[from] tokio_pg_mapper::Error),
}

impl error::ResponseError for CrenauxError {
    fn error_response(&self) -> HttpResponse {
        match self {
            CrenauxError::CreneauNotFound => HttpResponse::NotFound().json("crenaux not found"),
            CrenauxError::CreneauNotValid => HttpResponse::BadRequest().json("crenaux not valid"),
            CrenauxError::MapperError(_) => HttpResponse::InternalServerError().json("mapper error"),
        }
    }
}