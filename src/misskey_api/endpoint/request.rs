use serde::{de::DeserializeOwned, Serialize};
use super::api_target::ApiTarget;

pub async fn request<T, U>(host : &str, api_target : ApiTarget, param : &T) -> Result<U, Box<dyn std::error::Error>>
where
    T: Serialize,
    U : DeserializeOwned,
{
    let client = reqwest::Client::new();

    let url = format!("https://{}/api/{}", host, api_target.to_string());
    let res = client.post(url)
        .json(&param)
        .send()
        .await?;
    let return_obj : U = res.json::<U>().await?;

    Ok(return_obj)
}
