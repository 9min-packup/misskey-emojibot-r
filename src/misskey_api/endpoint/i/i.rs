use serde::Serialize;
use super::super::{request_json, ApiTarget};
use super::super::super::model::User;

#[derive(Debug, Serialize)]
struct Param {
    i : String
}

#[allow(dead_code)]
pub async fn i(host : &str, token : &str)-> Result<User, Box<dyn std::error::Error>> {
    let param : Param = Param{i : String::from(token)};
    let user : User = request_json::<Param, User>(&host, ApiTarget::I, &param).await?;
    Ok(user)
}