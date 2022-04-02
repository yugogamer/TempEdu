use actix_web::{post, web, Either, Responder, HttpResponse, cookie::Cookie};
use deadpool_postgres::Pool;

use crate::{entity::{auth::Login}, service::auth::login};



#[post("/auth")]
pub async fn road_login(pool: web::Data<Pool>, login_info: web::Json<Login>) -> Either<impl Responder, HttpResponse> {
    let conn = pool.get().await.unwrap();
    println!("{:?}", &login_info);
    let auth = login(&conn, &login_info.username, &login_info.password).await;
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