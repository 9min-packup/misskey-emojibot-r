use serde::Serialize;
use super::super::{request, ApiTarget};
use super::super::super::model::User;

#[derive(Debug, Serialize)]
struct Param {
    i: String
}

pub async fn i(host : &str, token : &str)-> Result<User, Box<dyn std::error::Error>> {
    let param: Param = Param{i : String::from(token)};
    let user : User = request::<Param, User>(&host, ApiTarget::I, &param).await?;
    Ok(user)
}