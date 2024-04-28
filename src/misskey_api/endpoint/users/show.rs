use serde::Serialize;
use super::super::{request_json, ApiTarget};
use super::super::super::model::User;

#[allow(non_snake_case)]
#[derive(Debug, Serialize)]
struct ByUserIdParam {
    userId : String
}

#[allow(dead_code)]
pub async fn show_by_user_id(host : &str, user_id : &str)-> Result<User, Box<dyn std::error::Error>> {
    let param : ByUserIdParam = ByUserIdParam{userId : String::from(user_id)};
    let user : User = request_json::<ByUserIdParam, User>(&host, ApiTarget::UsersShow, &param).await?;
    Ok(user)
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize)]
struct ByUsernameParam {
    username: String
}

#[allow(dead_code)]
pub async fn show_by_username(host : &str, username : &str)-> Result<User, Box<dyn std::error::Error>> {
    let param : ByUsernameParam = ByUsernameParam{username : String::from(username)};
    let user : User = request_json::<ByUsernameParam, User>(&host, ApiTarget::UsersShow, &param).await?;
    Ok(user)
}