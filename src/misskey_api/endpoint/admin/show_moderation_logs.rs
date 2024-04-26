use serde::Serialize;
use super::super::{request, ApiTarget};
use super::super::super::model::ModerationLog;

#[derive(Debug, Serialize)]
struct Param {
    i: String,
    limit: i32,
    r#type: Option<String>,
    userId: Option<String>,
}

pub async fn show_moderation_logs(host : &str, token : &str, limit : i32, r#type : Option<String>, user_id : Option<String>)-> Result<Vec<ModerationLog>, Box<dyn std::error::Error>> {
    let param: Param = Param
        {
            i : String::from(token),
            limit : limit,
            r#type : r#type,
            userId: user_id,
        };
    let moderation_logs : Vec<ModerationLog> = request::<Param, Vec<ModerationLog>>(&host, ApiTarget::AdminShowModerationLogs, &param).await?;
    Ok(moderation_logs)
}

#[derive(Debug, Serialize)]
struct BySinceIdParam {
    i: String,
    limit: i32,
    r#type: Option<String>,
    userId: Option<String>,
    sinceId: String,
}

pub async fn show_moderation_logs_by_since_id(host : &str, token : &str, limit : i32, r#type : Option<String>, user_id : Option<String>, since_id : &str)-> Result<Vec<ModerationLog>, Box<dyn std::error::Error>> {
    let param: BySinceIdParam = BySinceIdParam
        {
            i : String::from(token),
            limit : limit,
            r#type : r#type,
            userId: user_id,
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
    userId: Option<String>,
    untilId: String,
}

pub async fn show_moderation_logs_by_until_id(host : &str, token : &str, limit : i32, r#type : Option<String>, user_id : Option<String>, until_id : &str)-> Result<Vec<ModerationLog>, Box<dyn std::error::Error>> {
    let param: ByUntilIdParam = ByUntilIdParam
        {
            i : String::from(token),
            limit : limit,
            r#type : r#type,
            userId: user_id,
            untilId : String::from(until_id),
        };
    let moderation_logs : Vec<ModerationLog> = request::<ByUntilIdParam, Vec<ModerationLog>>(&host, ApiTarget::AdminShowModerationLogs, &param).await?;
    Ok(moderation_logs)
}

#[derive(Debug, Serialize)]
struct BySinceIdUntilIdParam {
    i: String,
    limit: i32,
    r#type: Option<String>,
    userId: Option<String>,
    untilId: String,
    sinceId: String,
}

pub async fn show_moderation_logs_by_until_id_since_id(host : &str, token : &str, limit : i32, r#type : Option<String>, user_id : Option<String>, until_id : &str, since_id : &str)-> Result<Vec<ModerationLog>, Box<dyn std::error::Error>> {
    let param: BySinceIdUntilIdParam = BySinceIdUntilIdParam
        {
            i : String::from(token),
            limit : limit,
            r#type : r#type,
            userId: user_id,
            untilId : String::from(until_id),
            sinceId : String::from(since_id),
        };
    let moderation_logs : Vec<ModerationLog> = request::<BySinceIdUntilIdParam, Vec<ModerationLog>>(&host, ApiTarget::AdminShowModerationLogs, &param).await?;
    Ok(moderation_logs)
}