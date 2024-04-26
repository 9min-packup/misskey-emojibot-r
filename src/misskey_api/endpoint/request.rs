use reqwest;
use serde::{Serialize, de::DeserializeOwned};
use super::api_target::ApiTarget;
use log::{trace, debug};

#[derive(Debug)]
pub struct RequestFailure{
    starus_code : Option<u16>,
    message : String
}

impl std::error::Error for RequestFailure {}

impl std::fmt::Display for RequestFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "request failue. \nmessage: \n{}", self.message)
    }
}


pub async fn request<T, U>(host : &str, api_target : ApiTarget, param : &T) -> Result<U, Box<dyn std::error::Error>>
where
    T: Serialize,
    U : DeserializeOwned,
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
        Err(e) => return Err( Box::new(RequestFailure{starus_code : None, message : e.to_string()})),
    };
    let status_code: u16  = res.status().as_u16();
    debug!("status_code {:?}", status_code);
    match status_code {
        200 => (),
        _ => return Err( Box::new(RequestFailure{starus_code : Some(status_code), message : res.text().await?})),
    }

    let return_obj : U = res.json::<U>().await?;

    Ok(return_obj)
}
