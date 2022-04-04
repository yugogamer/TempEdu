use actix_web::{post, web::{self}, Either, Responder, HttpResponse, cookie::Cookie};
use deadpool_postgres::Pool;

use crate::{entity::{auth::Login}, service::auth::login, utils::configuration::Configuration};



#[post("/auth")]
pub async fn road_login(pool: web::Data<Pool>, configuration : web::Data<Configuration> ,login_info: web::Json<Login>) -> Either<impl Responder, HttpResponse> {
    let conn = pool.get().await.unwrap();
    let auth = login(&conn, &login_info.username, &login_info.password, &configuration.key).await;
    match auth {
        Ok(token) => {
            let cookie = Cookie::build("session", token).secure(true).finish();

            return Either::Left(HttpResponse::Ok().cookie(cookie).finish());
        },
        Err(err) => {
            Either::Right(HttpResponse::BadRequest().body(err.to_string()))
        }
    }
}