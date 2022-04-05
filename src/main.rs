use std::{time::Duration};
use actix_web::{HttpServer, App, web::{self, Data}, middleware::Logger};
use actix_web_grants::GrantsMiddleware;

use crate::{utils::{configuration::Configuration, database::connection}, controller::base::status};

mod utils;
mod controller;
mod service;
mod entity;

#[tokio::main]
async fn main() -> Result<(), tokio_postgres::Error>{
    let config = Configuration::new();
    let connection = connection(&config).await?;
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("error"));

    let result = HttpServer::new(move ||{
        let auth = GrantsMiddleware::with_extractor(service::auth::extract);
        let config = Data::new(Configuration::new());

        App::new()
        .wrap(Logger::default())
        // Loading db
        .app_data(Data::new(connection.clone()))
        .app_data(config)
        // No protection road
        .service(status)
        .service(controller::auth::road_login)
        .service(
            web::scope("/api")
                        .service(web::scope("/v1")
                            .wrap(auth)
                            .service(web::scope("/user")
                                .service(controller::user::road_get_my_user)
                                .service(controller::user::road_get_user)
                                .service(controller::user::road_add_user)
                                )
                            )
                            .service(web::scope("/crenaux")
                                .service(controller::crenaux::get_my_edt)
                                .service(controller::crenaux::get_user_edt)
                                .service(controller::crenaux::get_groupe_edt)
                            )
        )
    })
    .keep_alive(Duration::from_secs(120))
    .bind((config.addresse, config.port)).expect("err : binding already use or not found")
    .run().await;

    if let Err(err) = result {
        eprintln!("{}", err);
    }
    Ok(())
}
