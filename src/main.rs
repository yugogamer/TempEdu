use actix_web::{HttpServer, App};

use crate::{utils::{configuration::Configuration, database::connection}, controller::base::status};

mod utils;
mod controller;

#[tokio::main]
async fn main() -> Result<(), tokio_postgres::Error>{
    let config = Configuration::new().await;
    println!("{:?}", &config);

    let connection = connection(&config).await?;
    
    HttpServer::new(move ||{
        App::new()
        .app_data(connection.clone())
        .service(status)
    })
    .bind((config.addresse.as_str(), config.port)).expect("err : no dispnible binding")
    .run().await;

    Ok(())
}
