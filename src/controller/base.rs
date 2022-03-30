use actix_web::{HttpRequest, get};



#[get("/")]
pub async fn status(_req: HttpRequest) -> &'static str{
    "ok"
}