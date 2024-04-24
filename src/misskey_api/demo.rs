use std::env;
use serde::{Deserialize, Serialize};
use log::{trace, debug};
use super::model;
use super::endpoint;

#[derive(Debug, Serialize)]
struct Param1 {
    i: String
}

#[derive(Serialize, Deserialize)]
struct Param2 {
    i: String,
    limit: i32,
    r#type: Option<String>
}


pub async fn hoge() -> Result<(), Box<dyn std::error::Error>> {
    let host: String = env::var("HOST").expect("HOST is not found in .env");
    let token: String = env::var("TOKEN").expect("TOKEN is not found in .env");

    let client: reqwest::Client = reqwest::Client::new();

    // 絵文字通知Botのアカウント情報を取得
    let url: String = format!("https://{}/api/{}", host, "i");
    let param1: Param1 = Param1 {i:token.clone()};
    let res: reqwest::Response = client.post(url)
        .json(&param1)
        .send()
        .await?;
    debug!("{:?}", res.status());
    let text : String = res.text().await?;
    trace!("{:?}", text);
    let user : model::User = serde_json::from_str(&text).unwrap();
    debug!("{:?}", user);

    let user_json : String = serde_json::to_string(&user).unwrap();
    debug!("{:?}", user_json);

    // モデレーションログを取得
    // let url2 = format!("https://{}/api/{}", host, "admin/show-moderation-logs");
    // let param2 = Param2 {i:token, limit:5, r#type: None};
    // let res2 = client.post(url2)
    //     .json(&param2)
    //     .send()
    //     .await?;
    // println!("{:?}", res2.status());
    // println!("{:?}", res2.text().await?);
    Ok(())
}

pub async fn hoge2()-> Result<(), Box<dyn std::error::Error>> {
    let host: String = env::var("HOST").expect("HOST is not found in .env");
    let token: String = env::var("TOKEN").expect("TOKEN is not found in .env");
    let param: Param1 = Param1{i : token};
    let user : model::User = endpoint::request::<Param1, model::User>(&host, endpoint::ApiTarget::I, &param).await?;

    debug!{"{:?}", user};

    Ok(())
}