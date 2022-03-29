use std::{fs::File, env};

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Configuration{
    pub addresse: String,
    pub port: u16,
    pub pg_string : String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoadedConfiguration{
    pub addresse: Option<String>,
    pub port: Option<u16>,
    pub pg_string : Option<String>,
}

impl Configuration{
    
    pub async fn new() -> Configuration{
        let mut config = Configuration{
            addresse: "127.0.0.1".to_owned(),
            port: 8080,
            pg_string: "postgres://api-edt:api_pswd@localhost/debug".to_owned(),
        };
        
        config.load_file().await;
        config.load_env().await;
        
        return config;
    }
    
    async fn load_file(&mut self){
        // load from config file
        let file = File::open("settings.yaml");
        if let Ok(file) = file{
            let yaml: Result<LoadedConfiguration, serde_yaml::Error> = serde_yaml::from_reader(&file);
            if let Ok(loaded_config) = yaml{
                if let Some(addresse) = loaded_config.addresse{
                    self.addresse = addresse;
                }
                
                if let Some(port) = loaded_config.port{
                    self.port = port;
                }
                
                if let Some(pg_string) = loaded_config.pg_string{
                    self.pg_string = pg_string;
                }
            }
        }
    }

    async fn load_env(&mut self){
        //loading from env variable
        if let Ok(addresse) = env::var("ADDRESSE"){
            self.addresse = addresse;
        }

        if let Ok(port) = env::var("PORT"){
            if let Ok(port) = port.parse(){
                self.port = port;
            }
        }

        if let Ok(pg_string) = env::var("PG_STRING"){
            self.pg_string = pg_string;
        }
    }
}

/*
impl fmt::Display for Configuration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Listening to : {}:{}\nPort", self.x, self.y)
    }
}
*/