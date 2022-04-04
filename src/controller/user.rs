use actix_web::{get, web, Responder, Result, post, HttpResponse, Either, HttpRequest};
use actix_web_grants::proc_macro::has_permissions;
use deadpool_postgres::{Pool};
use crate::{service::{user::{get_user, add_user}, auth::auth_user}, entity::user::UserInsertion};





#[get("/{id}")]
#[has_permissions("see_all_account")]
pub async fn road_get_user(pool: web::Data<Pool>, id: web::Path<i32>) -> Result<impl Responder> {
    let conn = pool.get().await.unwrap();
    let user = get_user(&conn, id.into_inner()).await?;
    Ok(web::Json(user))
}

#[get("")]
pub async fn road_get_my_user(req: HttpRequest) -> Result<impl Responder> {
    let cookies = req.cookie("session");
    let cookies = cookies.unwrap();

    let user = auth_user(cookies.value())?;

    Ok(web::Json(user))
}

#[post("")]
#[has_permissions("create_account")]
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