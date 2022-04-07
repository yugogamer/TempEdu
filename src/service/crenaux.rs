use actix_web::{error, HttpResponse};
use thiserror::Error;
use tokio_pg_mapper::FromTokioPostgresRow;
use tokio_postgres::Client;

use crate::entity::crenaux::{Crenau, InsertCrenaux};



pub async fn get_personal_creneaux_of_user(conn: &Client , id : i32, week : i32) -> Result<Vec<Crenau>, CrenauxError> {
    let rows = conn.query("
    SELECT C.id, C.id_week, C.id_matiere, C.start_time, C.end_time, C.description, C.name
    FROM creneaux C
    LEFT OUTER JOIN accountsToCreneaux AC ON AC.id_creneau = C.id
    WHERE
    C.id_week = $1 AND 
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
    SELECT C.id, C.id_week, C.id_matiere, C.start_time, C.end_time, C.description, C.name
    FROM creneaux C
    LEFT OUTER JOIN groupesToCreneaux GC ON GC.id_creneau = C.id
    WHERE
    C.id_week = $1 AND 
    GC.id_groupe = $2 AND"
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
    SELECT C.id, C.id_week, C.id_matiere, C.start_time, C.end_time, C.description, C.name
    FROM creneaux C
    LEFT OUTER JOIN groupesToCreneaux GC ON GC.id_creneau = C.id
    LEFT OUTER JOIN accountsToCreneaux AC ON AC.id_creneau = C.id
    WHERE
    C.id_week = $1 AND(
    ( 
        GC.id_groupe IN (
            SELECT id_groupe 
            FROM accountsToGroupes 
            WHERE id_account = $2
        ) 
    )
    OR
    (
        AC.id_account = $2
    ))"
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

pub async fn create_creneaux_groupe(conn: &Client , creneau : InsertCrenaux, id_groupe : i32) -> Result<(), CrenauxError>{
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
      ) RETURNING id", &[&creneau.id_week, &creneau.id_matiere, &creneau.start_time, &creneau.end_time, &creneau.name, &creneau.description]).await;
    if let Err(_err) = row {
        return Err(CrenauxError::CreneauNotValid);
    }
    let id_creneau : i32 = row.unwrap().get(0).unwrap().get("id");
    let row = conn.query(
        "INSERT INTO groupesToCreneaux(id_creneau, id_groupe)
        VALUES($1, $2)", &[&id_creneau, &id_groupe]).await;
    if let Err(_err) = row {
        return Err(CrenauxError::CreneauNotValid);
    }
    Ok(())
}

pub async fn create_creneaux_user(conn: &Client , creneau : InsertCrenaux, id_account : i32) -> Result<(), CrenauxError>{
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
      ) RETURNING id", &[&creneau.id_week, &creneau.id_matiere, &creneau.start_time, &creneau.end_time, &creneau.name, &creneau.description]).await;
    if let Err(_err) = row {
        return Err(CrenauxError::CreneauNotValid);
    }
    let id_creneau : i32 = row.unwrap().get(0).unwrap().get("id");
    let row = conn.query(
        "INSERT INTO accountsToCreneaux(id_creneau, id_account)
        VALUES($1, $2)", &[&id_creneau, &id_account]).await;
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
    #[error("groupe not valid")]
    GroupeNotValid,
}

impl error::ResponseError for CrenauxError {
    fn error_response(&self) -> HttpResponse {
        match self {
            CrenauxError::CreneauNotFound => HttpResponse::BadRequest().json("crenaux not found"),
            CrenauxError::CreneauNotValid => HttpResponse::BadRequest().json("crenaux not valid"),
            CrenauxError::MapperError(_) => HttpResponse::InternalServerError().json("mapper error"),
            CrenauxError::GroupeNotValid => HttpResponse::BadRequest().json("groupe not valid"),
        }
    }
}