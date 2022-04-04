use core::fmt;
use std::{fs::File, env};
use hmac::Hmac;
use serde::{Serialize, Deserialize};
use sha2::Sha384;
use hmac::digest::KeyInit;
#[derive(Debug, Clone)]
pub struct Configuration{
    pub addresse: String,
    pub port: u16,
    pub pg_string : String,
    pub jwt_secret : String,
    pub key : Hmac<Sha384>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoadedConfiguration{
    pub addresse: Option<String>,
    pub port: Option<u16>,
    pub pg_string : Option<String>,
    pub jwt_secret : Option<String>,
}

impl Configuration{
    
    pub fn new() -> Configuration{
        let mut config = Configuration{
            addresse: "127.0.0.1".to_owned(),
            port: 8080,
            pg_string: "postgres://api-edt:api_pswd@localhost/debug".to_owned(),
            jwt_secret: "secret".to_owned(),
            key: Hmac::new_from_slice(b"secret").unwrap(),
        };
        
        config.load_file();
        config.load_env();
        
        config.key = Hmac::new_from_slice(config.jwt_secret.as_bytes()).unwrap();

        config
    }
    
    fn load_file(&mut self){
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

                if let Some(jwt_secret) = loaded_config.jwt_secret{
                    self.jwt_secret = jwt_secret;
                }
            }
        }
    }

    fn load_env(&mut self){
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

        if let Ok(jwt_secret) = env::var("JWT_SECRET"){
            self.jwt_secret = jwt_secret;
        }
    }
}


impl fmt::Display for Configuration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Listening to : {}:{}", self.addresse, self.port)
    }
}
