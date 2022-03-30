use chrono::NaiveDateTime;
use tokio_pg_mapper::FromTokioPostgresRow;
use tokio_postgres::{Client, types::Timestamp};
use thiserror::Error;



use crate::entity::user::User;

#[derive(Debug, Error)]
pub enum AuthError{
    #[error("username or password inccorect")]
    UserNotFoundOrPasswordNotFound,
    #[error("session expired")]
    SessionExpired,
    #[error("session not found")]
    NoSession,
    #[error("Error api")]
    MapperError(#[from] tokio_pg_mapper::Error),
    #[error("Error api")]
    DbError(#[from] tokio_postgres::Error),
}


pub async fn login(conn: &Client, username: &str, password: &str) -> Result<String, AuthError> {
    let row = conn.query_one("SELECT * FROM accounts WHERE username = $1 AND password = crypt($2, password)", &[&username, &password]).await;
    if let Err(_err) = row {
        return Err(AuthError::UserNotFoundOrPasswordNotFound);
    }

    let user = User::from_row(row.unwrap())?;

    let _row = conn.query("DELETE FROM session WHERE id_user = $1", &[&user.id]).await?;
    let row = conn.query_one("INSERT INTO session (id_user) VALUES ($1) return id", &[&user.id]).await?;

    Ok(row.get("id"))
}

pub async fn auth_user(conn: &Client, id: &str) -> Result<User, AuthError> {
    let row = conn.query_one("SELECT A.id A.username, A.first_name, A.last_name, A.abreviate_name, A.mail, S.expiration_date FROM accounts as A, session as S WHERE S.id = $1", &[&id]).await;
    if let Err(_err) = row {
        return Err(AuthError::NoSession);
    }
    let row = row.unwrap();

    let timestamp : NaiveDateTime;
    timestamp = row.get("expiration_date");
    if timestamp.timestamp_nanos() < chrono::Utc::now().timestamp_nanos(){
        return Err(AuthError::SessionExpired);
    }

    let user = User::from_row(row)?;

    Ok(user)
}