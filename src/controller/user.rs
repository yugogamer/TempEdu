use std::str::FromStr;

use actix_web::{get, web, Responder, Result, post, HttpResponse, Either, HttpRequest};
use deadpool_postgres::{Pool};
use crate::{service::user::{get_user, add_user, get_user_by_session}, entity::user::UserInsertion};





#[get("/{id}")]
pub async fn road_get_user(pool: web::Data<Pool>, id: web::Path<i32>) -> Result<impl Responder> {
    let conn = pool.get().await.unwrap();
    let user = get_user(&conn, id.into_inner()).await?;
    Ok(web::Json(user))
}

#[get("/")]
pub async fn road_get_my_user(pool: web::Data<Pool>, req: HttpRequest) -> Result<impl Responder> {
    let conn = pool.get().await.unwrap();

    let uuid = uuid::Uuid::from_str(req.cookie("session").unwrap().value()).unwrap();

    let user = get_user_by_session(&conn, uuid).await?;
    Ok(web::Json(user))
}

#[post("")]
pub async fn road_add_user(pool: web::Data<Pool>, user: web::Json<UserInsertion>) -> Either<impl Responder, HttpResponse> {
    let conn = pool.get().await.unwrap();
    let user = add_user(&conn, &*user).await;
    match user {
        Ok(user) => Either::Left(web::Json(user)),
        Err(err) => {
            Either::Right(HttpResponse::BadRequest().body(err.to_string()))
        }
    }
}