

#[post("")]
pub async fn road_login(pool: web::Data<Pool>, user: web::Json<UserInsertion>) -> Either<impl Responder, HttpResponse> {
    let conn = pool.get().await.unwrap();
    let user = add_user(&conn, &*user).await;
    match user {
        Ok(user) => Either::Left(web::Json(user)),
        Err(err) => {
            Either::Right(HttpResponse::BadRequest().body(err.to_string()))
        }
    }
}