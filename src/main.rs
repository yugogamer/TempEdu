use std::time::Duration;

use actix_web::{HttpServer, App, web::{self, Data}};

use crate::{utils::{configuration::Configuration, database::connection}, controller::base::status};

mod utils;
mod controller;
mod service;
mod entity;

#[tokio::main]
async fn main() -> Result<(), tokio_postgres::Error>{
    let config = Configuration::new().await;
    println!("{:?}", &config);

    let connection = connection(&config).await?;
    
    let result = HttpServer::new(move ||{
        App::new()
        .app_data(Data::new(connection.clone()))
        .service(status)
        .service(
            web::scope("/api")
                        .service(web::scope("/v1")
                            .service(web::scope("/user")
                                .service(controller::user::road_get_user)
                                .service(controller::user::road_add_user)
                                )
                            )
        )
    })
    .keep_alive(Duration::from_secs(120))
    .bind((config.addresse.as_str(), config.port)).expect("err : binding already use or not found")
    .run().await;

    if let Err(err) = result {
        eprintln!("{}", err);
    }
    Ok(())
}
