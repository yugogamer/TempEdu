use crate::utils::{configuration::Configuration, database::connection};

mod utils;

#[tokio::main]
async fn main() -> Result<(), tokio_postgres::Error>{
    let config = Configuration::new().await;
    println!("{:?}", &config);

    let connection = connection(&config).await?;
    
    Ok(())
}
