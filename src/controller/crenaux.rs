use actix_web::{get, web, HttpRequest, Responder};
use actix_web_grants::proc_macro::has_any_permission;
use deadpool_postgres::Pool;

use crate::{utils::configuration::Configuration, service::{auth::auth_user, crenaux::{get_creneaux_of_user_with_groupe, get_creneaux_of_groupe}}};




#[get("/{id_week}")]
pub async fn get_my_edt(pool: web::Data<Pool>, configuration : web::Data<Configuration> , req: HttpRequest, id_week: web::Path<i32>) -> Result<impl Responder, actix_web::Error> {
    let conn = pool.get().await.unwrap();

    let cookies = req.cookie("session");
    let cookies = cookies.unwrap();

    let user = auth_user(cookies.value(), &configuration.key)?;

    let edt = get_creneaux_of_user_with_groupe(&conn, user.id, *id_week).await?;

    Ok(web::Json(edt))
}

#[get("/{id_week}/{id_user}")]
#[has_any_permission("edit_edt")]
pub async fn get_user_edt(pool: web::Data<Pool>, id_week: web::Path<i32>, id_user: web::Path<i32>) -> Result<impl Responder, actix_web::Error> {
    let conn = pool.get().await.unwrap();

    let edt = get_creneaux_of_user_with_groupe(&conn, *id_user, *id_week).await?;

    Ok(web::Json(edt))
}

#[get("/{id_week}/g/{id_groupe}")]
pub async fn get_groupe_edt(pool : web::Data<Pool>, id_week: web::Path<i32>, id_groupe: web::Path<i32>) -> Result<impl Responder, actix_web::Error> {
    let conn = pool.get().await.unwrap();

    let edt = get_creneaux_of_groupe(&conn, *id_groupe, *id_week).await?;

    Ok(web::Json(edt))
}