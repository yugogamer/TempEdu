use std::str::FromStr;

use tokio_postgres::{NoTls, Client};
use deadpool_postgres::{Manager, ManagerConfig, Pool, RecyclingMethod};

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