use std::env;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
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
    let host = env::var("HOST").expect("HOST is not found in .env");
    let token = env::var("TOKEN").expect("TOKEN is not found in .env");

    let client = reqwest::Client::new();

    // 絵文字通知Botのアカウント情報を取得
    let url = format!("https://{}/api/{}", host, "i");
    let param1 = Param1 {i:token.clone()};
    let res = client.post(url)
        .json(&param1)
        .send()
        .await?;
    println!("{:?}", res.status());
    println!("{:?}", res.text().await?);

    // モデレーションログを取得
    let url2 = format!("https://{}/api/{}", host, "admin/show-moderation-logs");
    let param2 = Param2 {i:token, limit:5, r#type: None};
    let res2 = client.post(url2)
        .json(&param2)
        .send()
        .await?;
    println!("{:?}", res2.status());
    println!("{:?}", res2.text().await?);
    Ok(())
}