use std::str::FromStr;

use actix_web::{dev::ServiceRequest, web};
use chrono::NaiveDateTime;
use deadpool_postgres::{Pool, PoolError};
use tokio_pg_mapper::FromTokioPostgresRow;
use tokio_postgres::{Client, types::Timestamp};
use thiserror::Error;
use uuid::Uuid;



use crate::entity::user::User;

#[derive(Debug, Error)]
pub enum AuthError{
    #[error("username or password inccorect")]
    UserNotFoundOrPasswordNotFound,
    #[error("session expired")]
    SessionExpired,
    #[error("session not found")]
    NoSession,
    #[error("Error api : mapper")]
    MapperError(#[from] tokio_pg_mapper::Error),
    #[error("Error api : database")]
    DbError(#[from] tokio_postgres::Error),
    #[error("Error api : database pool")]
    PoolError(#[from] PoolError),
    #[error("Fake UUID")]
    UuidError(#[from] uuid::Error)
}


pub async fn login(conn: &Client, username: &str, password: &str) -> Result<String, AuthError> {
    let row = conn.query_one("SELECT * FROM accounts WHERE username = $1 AND password = crypt($2, password)", &[&username, &password]).await;
    if let Err(_err) = row {
        return Err(AuthError::UserNotFoundOrPasswordNotFound);
    }

    let user = User::from_row(row.unwrap())?;

    let _row = conn.query("DELETE FROM session WHERE id_user = $1", &[&user.id]).await;
    let row = conn.query_one("INSERT INTO session (id_user) VALUES ($1) RETURNING id", &[&user.id]).await?;

    let uuid: Uuid = row.get("id");

    Ok(uuid.to_string())
}

pub async fn auth_user(conn: &Client, id: &str) -> Result<User, AuthError> {
    let uuid = Uuid::from_str(id)?;
    let row = conn.query_one("SELECT A.id, A.username, A.first_name, A.last_name, A.abreviate_name, A.mail, S.expiration_date FROM accounts as A, session as S WHERE S.id = $1", &[&uuid]).await;
    if let Err(_err) = row {
        return Err(AuthError::NoSession);
    }
    let row = row.unwrap();

    let timestamp : chrono::DateTime<chrono::Utc>;
    timestamp = row.get("expiration_date");

    let now = chrono::Utc::now();

    if timestamp.timestamp_nanos() < now.timestamp_nanos(){
        return Err(AuthError::SessionExpired);
    }

    let _update = conn.query("UPDATE session SET expiration_date = NOW() + INTERVAL '7 day' WHERE id = $1", &[&id]).await;
    
    let user = User::from_row(row)?;

    Ok(user)
}

pub async fn extract(req: &ServiceRequest) -> Result<Vec<String>, actix_web::Error> {
    let pool = req.app_data::<web::Data<Pool>>().unwrap();

    let conn = pool.get().await;
    if let Err(err) = conn{
        return Err(AuthError::PoolError(err).into());
    }
    let conn = conn.unwrap();

    let cookies = req.cookie("session");
    if cookies.is_none() {
        return Err(AuthError::NoSession.into());
    }
    let cookies = cookies.unwrap();
    
    let user = auth_user(&conn, &cookies.value()).await?;


    Ok(vec![user.username.clone()])
}

 
impl From<AuthError> for actix_web::Error {
    fn from(err: AuthError) -> Self {
        actix_web::error::ErrorInternalServerError(err.to_string())
    }
}
