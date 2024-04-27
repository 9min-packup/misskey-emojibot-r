use reqwest::{self, Response};
use serde::{Serialize, de::DeserializeOwned};
use super::api_target::ApiTarget;
use log::{trace, debug};

#[allow(dead_code)]
#[derive(Debug)]
pub struct RequestFailure{
    starus_code : Option<u16>,
    message : String
}

impl std::error::Error for RequestFailure {}

impl std::fmt::Display for RequestFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "request failrue. \nmessage: \n{}", self.message)
    }
}

pub async fn request<T>(host : &str, api_target : ApiTarget, param : &T) -> Result<Response, Box<dyn std::error::Error>>
where
    T: Serialize,
{
    let client = reqwest::Client::new();

    let url = format!("{}/api/{}", host, ApiTarget::to_string(&api_target));
    let res_result = client.post(url)
        .json(&param)
        .send()
        .await;
    trace!("{:?}", res_result);
    let res = match res_result {
        Ok(res) => res,
        Err(e) => return Err(Box::new(RequestFailure{starus_code : None, message : e.to_string()})),
    };
    debug!("status_code {:?}", res.status().as_u16());
    let status_code: u16  = res.status().as_u16();
    match status_code {
        200 => (),
        _ => return Err(Box::new(RequestFailure{starus_code : Some(status_code), message : res.text().await?})),
    }

    Ok(res)
}

#[allow(dead_code)]
pub async fn to_json<T>(res : Response) -> Result<T, Box<dyn std::error::Error>> 
where 
    T: DeserializeOwned,
{
    let obj : T = match res.json::<T>().await {
        Ok(x) => x,
        Err(e) => return Err(Box::new(e)),
    };

    Ok(obj)
}

#[allow(dead_code)]
pub async fn request_text<T>(host : &str, api_target : ApiTarget, param : &T) -> Result<String, Box<dyn std::error::Error>>
where
    T: Serialize,
{
    let res = match request::<T>(host, api_target, param).await {
        Ok(x) => x,
        Err(e) => return Err(e),
    };
    let text = match res.text().await {
        Ok(x) => x,
        Err(e) => return Err(Box::new(e)),
    };

    Ok(text)
}

pub async fn request_json<T, U>(host : &str, api_target : ApiTarget, param : &T) -> Result<U, Box<dyn std::error::Error>>
where
    T: Serialize,
    U: DeserializeOwned,
{
    let res = match request::<T>(host, api_target, param).await {
        Ok(x) => x,
        Err(e) => return Err(e),
    };
    let obj : U = match to_json::<U>(res).await {
        Ok(x) => x,
        Err(e) => return Err(e),
    };

    Ok(obj)
}