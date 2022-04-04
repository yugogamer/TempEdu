use std::str::FromStr;

use tokio_postgres::{NoTls, Client};
use deadpool_postgres::{Manager, ManagerConfig, Pool, RecyclingMethod};

use crate::{service::{user::{get_user, add_user}, role::set_role}, entity::user::UserInsertion};

use super::configuration::Configuration;



pub async fn connection(config : &Configuration) -> Result<Pool, tokio_postgres::Error>{
    let pg_config = tokio_postgres::Config::from_str(&config.pg_string).expect("Error : not a valid connection string");

    let mgr_config = ManagerConfig {
        recycling_method: RecyclingMethod::Verified
    };

    let mgr = Manager::from_config(pg_config, NoTls, mgr_config);
    let pool = Pool::builder(mgr).max_size(16).build().unwrap();

    let mut client = pool.get().await.unwrap();

    if let Err(err) = make_migration(&mut client).await{
        eprintln!("migration error : {}", err);
    }

    let conn = pool.get().await.unwrap();

    let user = get_user(&conn, 1).await;
    match user {
        Ok(_user) => {
            println!("admin already exist");
        },
        Err(_err) => {
            println!("Creating admin");
            let user = add_user(&conn, &UserInsertion { 
                username: "admin".to_string(), 
                mdp: "admin".to_string(), 
                first_name: "admin".to_string(), 
                last_name: "admin".to_string(), 
                abreviate_name: "admin".to_string(), 
                mail: "adming@exemple.com".to_string() }).await;
            set_role(&conn, user.unwrap().id, 1).await;
            println!("admin created");
        }
    }

    Ok(pool)
}

mod embedded {
    use refinery::embed_migrations;
    embed_migrations!("./migrations/");
}


async fn make_migration(conn : &mut Client) -> Result<(), refinery::Error>{
    embedded::migrations::runner().run_async(conn).await?;
    Ok(())
}