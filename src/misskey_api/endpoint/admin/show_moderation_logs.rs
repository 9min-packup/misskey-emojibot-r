use serde::Serialize;
use crate::misskey_api::endpoint::{request, ApiTarget};
use crate::misskey_api::model::ModerationLog;

#[derive(Debug, Serialize)]
struct Param {
    i: String,
    limit: i32,
    r#type: Option<String>,
}

pub async fn show_moderation_logs(host : &str, token : &str, limit : i32, r#type : Option<String>)-> Result<Vec<ModerationLog>, Box<dyn std::error::Error>> {
    let param: Param = Param
        {
            i : String::from(token),
            limit : limit,
            r#type : r#type,
        };
    let moderation_logs : Vec<ModerationLog> = request::<Param, Vec<ModerationLog>>(&host, ApiTarget::AdminShowModerationLogs, &param).await?;
    Ok(moderation_logs)
}

#[derive(Debug, Serialize)]
struct BySinceIdParam {
    i: String,
    limit: i32,
    r#type: Option<String>,
    sinceId: String,
}

pub async fn show_moderation_logs_by_since_id(host : &str, token : &str, limit : i32, r#type : Option<String>, since_id : &str)-> Result<Vec<ModerationLog>, Box<dyn std::error::Error>> {
    let param: BySinceIdParam = BySinceIdParam
        {
            i : String::from(token),
            limit : limit,
            r#type : r#type,
            sinceId : String::from(since_id),
        };
    let moderation_logs : Vec<ModerationLog> = request::<BySinceIdParam, Vec<ModerationLog>>(&host, ApiTarget::AdminShowModerationLogs, &param).await?;
    Ok(moderation_logs)
}

#[derive(Debug, Serialize)]
struct ByUntilIdParam {
    i: String,
    limit: i32,
    r#type: Option<String>,
    untilId: String,
}

pub async fn show_moderation_logs_by_until_id(host : &str, token : &str, limit : i32, r#type : Option<String>, until_id : &str)-> Result<Vec<ModerationLog>, Box<dyn std::error::Error>> {
    let param: ByUntilIdParam = ByUntilIdParam
        {
            i : String::from(token),
            limit : limit,
            r#type : r#type,
            untilId : String::from(until_id),
        };
    let moderation_logs : Vec<ModerationLog> = request::<ByUntilIdParam, Vec<ModerationLog>>(&host, ApiTarget::AdminShowModerationLogs, &param).await?;
    Ok(moderation_logs)
}