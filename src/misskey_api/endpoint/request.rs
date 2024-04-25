use reqwest;
use serde::{Serialize, de::DeserializeOwned};
use super::api_target::ApiTarget;
use log::trace;

pub async fn request<T, U>(host : &str, api_target : ApiTarget, param : &T) -> Result<U, Box<dyn std::error::Error>>
where
    T: Serialize,
    U : DeserializeOwned,
{
    let client = reqwest::Client::new();

    let url = format!("{}/api/{}", host, ApiTarget::to_string(&api_target));
    let res = client.post(url)
        .json(&param)
        .send()
        .await?;
    let return_obj : U = res.json::<U>().await?;

    Ok(return_obj)
}
