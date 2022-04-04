use serde::{Deserialize, Serialize};

use super::user::User;

#[derive(Debug, Serialize, Deserialize)]
pub struct Login {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserInformation{
    pub id: i32,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub abreviate_name: String,
    pub mail: String,
    pub permission_list: Vec<String>,
}

impl UserInformation {
    
    pub fn new(user : User, permision : Vec<String>) -> Self{
        Self{
            id : user.id,
            username : user.username,
            first_name : user.first_name,
            last_name : user.last_name,
            abreviate_name : user.abreviate_name,
            mail : user.mail,
            permission_list : permision,
        }
    }
}