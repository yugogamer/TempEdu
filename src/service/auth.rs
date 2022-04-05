use actix_web::{dev::ServiceRequest, web::Data, error};
use deadpool_postgres::{PoolError};
use tokio_pg_mapper::FromTokioPostgresRow;
use tokio_postgres::{Client};
use thiserror::Error;
use hmac::{Hmac, digest::InvalidLength};
use jwt::{AlgorithmType, Header, SignWithKey, Token, VerifyWithKey};
use sha2::Sha384;



use crate::{entity::{user::User, auth::UserInformation}, utils::configuration::Configuration};

use super::user::{get_permission_list, UserError};

#[derive(Debug, Error)]
pub enum AuthError{
    #[error("username or password inccorect")]
    UserNotFoundOrPasswordNotFound,
    #[error("session not found")]
    NoSession,
    #[error("Error api : mapper")]
    MapperError(#[from] tokio_pg_mapper::Error),
    #[error("Error api : database : `{0}`")]
    DbError(#[from] tokio_postgres::Error),
    #[error("Error api : database pool")]
    PoolError(#[from] PoolError),
    #[error("Fake UUID")]
    UuidError(#[from] uuid::Error),
    #[error("Error api : user not found")]
    UserNotFound(#[from] UserError),
    #[error("Error api : KeyError")]
    KeyError(#[from] InvalidLength),
    #[error("Error api : JWTError")]
    JWTError(#[from] jwt::Error),
}


pub async fn login(conn: &Client, username: &str, password: &str, key : &Hmac<Sha384> ) -> Result<String, AuthError> {
    let row = conn.query_one("SELECT * FROM accounts WHERE username = $1 AND password = crypt($2, password)", &[&username, &password]).await;
    if let Err(_err) = row {
        return Err(AuthError::UserNotFoundOrPasswordNotFound);
    }

    let header = Header {
        algorithm: AlgorithmType::Hs384,
        ..Default::default()
    };

    let user = User::from_row(row.unwrap())?;
    let permission_list = get_permission_list(conn, user.id).await?;
    let user = UserInformation::new(user, permission_list);

    let token = Token::new(header, user).sign_with_key(key)?;

    Ok(token.as_str().to_string())
}

pub fn auth_user(jwt: &str, key : &Hmac<Sha384>) -> Result<UserInformation, AuthError> {
    let token : Token<Header, UserInformation, _> = jwt.verify_with_key(key)?;
    let user = token.claims().clone();

    Ok(user)
}

pub async fn extract(req: &ServiceRequest) -> Result<Vec<String>, actix_web::Error> {
    let configuration = req.app_data::<Data<Configuration>>().unwrap();
    let cookies = req.cookie("session");
    if cookies.is_none() {
        return Err(AuthError::NoSession.into());
    }
    let cookies = cookies.unwrap();
    
    let user = auth_user( cookies.value(), &configuration.key)?;

    Ok(user.permission_list)
}

 impl From<UserError> for actix_web::Error {
    fn from(err: UserError) -> Self {
        actix_web::error::ErrorInternalServerError(err.to_string())
    }
}

impl error::ResponseError for AuthError {
    fn error_response(&self) -> actix_web::HttpResponse {
        match self {
            AuthError::UserNotFoundOrPasswordNotFound => actix_web::HttpResponse::Unauthorized().body("username or password inccorect"),
            AuthError::NoSession => actix_web::HttpResponse::Unauthorized().body("session not found"),
            AuthError::MapperError(_err) => actix_web::HttpResponse::InternalServerError().body("Error api : mapper"),
            AuthError::DbError(_err) => actix_web::HttpResponse::InternalServerError().body("Error api : database"),
            AuthError::PoolError(_err) => actix_web::HttpResponse::InternalServerError().body("Error api : database pool"),
            AuthError::UuidError(_err) => actix_web::HttpResponse::InternalServerError().body("Error api : user not found"),
            AuthError::UserNotFound(_err) => actix_web::HttpResponse::InternalServerError().body("Error api : user not found"),
            AuthError::KeyError(_err) => actix_web::HttpResponse::InternalServerError().body("Error api : KeyError"),
            AuthError::JWTError(_err) => actix_web::HttpResponse::InternalServerError().body("Error api : JWTError"),
        }
    }
}
