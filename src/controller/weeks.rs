use actix_web::{post, web, Responder, HttpResponse};
use actix_web_grants::proc_macro::has_permissions;
use deadpool_postgres::Pool;

use crate::{entity::weeks::WeekInsertion, service::weeks::create_weeks};




#[post("")]
#[has_permissions("edit_edt")]
pub async fn add_weeks(pool: web::Data<Pool>, week: web::Json<WeekInsertion>) -> Result<impl Responder, actix_web::Error> {
    let conn = pool.get().await.unwrap();

    create_weeks(&conn, week.iso_string.clone()).await;

    return Ok(HttpResponse::Ok());
}