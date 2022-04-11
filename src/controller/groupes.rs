use actix_web::{get, web, Responder, Result, post, HttpResponse, Either};
use actix_web_grants::proc_macro::has_permissions;
use deadpool_postgres::Pool;

use crate::{service::groupe::{get_all_groupes_unprotected, get_all_groupes, set_user_to_groupe}, entity::groupes::InsertGroupe};



#[post("")]
#[has_permissions("edit_edt")]
pub async fn create_groupe(pool: web::Data<Pool>, groupe: web::Json<InsertGroupe>) -> Either<impl Responder, HttpResponse> {
    let conn = pool.get().await.unwrap();
    let result = crate::service::groupe::create_groupe(&conn, groupe.0).await;
    match result {
        Ok(_) => Either::Left(web::Json(())),
        Err(err) => {
            Either::Right(HttpResponse::BadRequest().body(err.to_string()))
        }
    }
}

#[get("")]
pub async fn get_groupes_unprotected(pool: web::Data<Pool>) -> Result<impl Responder>{
    let conn = pool.get().await.unwrap();
    let groupes = get_all_groupes_unprotected(&conn).await?;
    Ok(web::Json(groupes))
}

#[get("")]
#[has_permissions("edit_edt")]
pub async fn get_groupes(pool: web::Data<Pool>) -> Result<impl Responder>{
    let conn = pool.get().await.unwrap();
    let groupes = get_all_groupes(&conn).await?;
    Ok(web::Json(groupes))
}

#[post("/{id_groupe}/{id_user}")]
#[has_permissions("edit_edt")]
pub async fn set_groupes(pool: web::Data<Pool>, id_user: web::Path<i32>, id_groupe: web::Path<i32>) -> Result<impl Responder>{
    let conn = pool.get().await.unwrap();
    set_user_to_groupe(&conn, *id_user, *id_groupe).await?;
    Ok(HttpResponse::Accepted())
}