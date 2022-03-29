use tokio_postgres::{NoTls, Client};

use super::configuration::Configuration;



pub async fn connection(config : &Configuration) -> Result<Client, tokio_postgres::Error>{
    let (mut client, connection) =
        tokio_postgres::connect(config.pg_string.as_str(), NoTls).await?;
    tokio::spawn(async move {
        if let Err(err) = connection.await {
            eprintln!("connection error: {}", err);
        }
    });

    if let Err(err) = make_migration(&mut client).await{
        eprintln!("migration error : {}", err);
    }

    Ok(client)
}

mod embedded {
    use refinery::embed_migrations;
    embed_migrations!("./migrations/");
}


async fn make_migration(conn : &mut Client) -> Result<(), refinery::Error>{
    embedded::migrations::runner().run_async(conn).await?;
    Ok(())
}