use std::time::Duration;

use actix_web::{HttpServer, App, web};

use crate::{utils::{configuration::Configuration, database::connection}, controller::base::status};

mod utils;
mod controller;

#[tokio::main]
async fn main() -> Result<(), tokio_postgres::Error>{
    let config = Configuration::new().await;
    println!("{:?}", &config);

    let connection = connection(&config).await?;
    
    let result = HttpServer::new(move ||{
        App::new()
        .app_data(connection.clone())
        .service(status)
        .service(
            web::scope("/api").service(web::scope("/v1"))
        )
    })
    .keep_alive(Duration::from_secs(120))
    .bind((config.addresse.as_str(), config.port)).expect("err : no dispnible binding")
    .run().await;

    if let Err(err) = result {
        eprintln!("{}", err);
    }
    Ok(())
}
