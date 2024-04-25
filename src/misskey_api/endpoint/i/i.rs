use serde::Serialize;
use crate::misskey_api::endpoint::{request, ApiTarget};
use crate::misskey_api::model::User;

#[derive(Debug, Serialize)]
struct Param {
    i: String
}

pub async fn i(host : &str, token : &str)-> Result<User, Box<dyn std::error::Error>> {
    let param: Param = Param{i : String::from(token)};
    let user : User = request::<Param, User>(&host, ApiTarget::I, &param).await?;
    Ok(user)
}