use serde::Serialize;
use crate::misskey_api::endpoint::{request, ApiTarget};
use crate::misskey_api::model::User;

#[derive(Debug, Serialize)]
struct ByUserIdParam {
    userId: String
}

pub async fn show_by_user_id(host : &str, user_id : &str)-> Result<User, Box<dyn std::error::Error>> {
    let param: ByUserIdParam = ByUserIdParam{userId : String::from(user_id)};
    let user : User = request::<ByUserIdParam, User>(&host, ApiTarget::UsersShow, &param).await?;
    Ok(user)
}

#[derive(Debug, Serialize)]
struct ByUsernameParam {
    username: String
}

pub async fn show_by_username(host : &str, username : &str)-> Result<User, Box<dyn std::error::Error>> {
    let param: ByUsernameParam = ByUsernameParam{username : String::from(username)};
    let user : User = request::<ByUsernameParam, User>(&host, ApiTarget::UsersShow, &param).await?;
    Ok(user)
}