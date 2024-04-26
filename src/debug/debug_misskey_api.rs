use std::env;
use log::{trace, debug};
use serde::Serialize;
use crate::util::{get_env, get_string_env};
use crate::misskey_api::endpoint;
use crate::misskey_api::model;

#[derive(Debug, Serialize)]
struct Param{
    i : String,
}

pub async fn debug_misskey_api() -> Result<(), Box<dyn std::error::Error>>  {
    let debug_endpoint: String = env::var("DEBUG_MI_ENDPOINT").expect("DEBUG_MI_ENDPOINT : invalid endpoint");
    match &debug_endpoint.as_str() {
        &"request" => debug_request().await?,
        &"i" => debug_i().await?,
        &"users_show_by_user_id" => debug_users_show_by_user_id().await?,
        &"users_show_by_username" => debug_users_show_by_username().await?,
        &"admin_show_moderation_logs" => debug_admin_show_moderation_logs().await?,
        &"admin_show_moderation_logs_by_since_id" => debug_admin_show_moderation_logs_by_since_id().await?,
        &"admin_show_moderation_logs_by_until_id" => debug_admin_show_moderation_logs_by_until_id().await?,
        &"admin_show_moderation_logs_by_until_id_since_id" => debug_admin_show_moderation_logs_by_until_id_since_id().await?,
        _ => (),
    }

    Ok(())
}

pub async fn debug_request()-> Result<(), Box<dyn std::error::Error>> {
    debug!{"debug request"};
    let host: String = get_string_env("DEBUG_MI_HOST", "https://example.tld");
    let token: String = get_string_env("DEBUG_MI_TOKEN", "XXXXXXXXXXXXXXXX");
    let param: Param = Param{i : token};
    let user : model::User = endpoint::request::<Param, model::User>(&host, endpoint::ApiTarget::I, &param).await?;

    debug!{"{:?}", user};

    Ok(())
}

pub async fn debug_i()-> Result<(), Box<dyn std::error::Error>> {
    debug!{"debug i"};
    let host: String = get_string_env("DEBUG_MI_HOST", "https://example.tld");
    let token: String = get_string_env("DEBUG_MI_TOKEN", "XXXXXXXXXXXXXXXX");
    let user : model::User = endpoint::i::i(&host, &token).await?;
    debug!{"{:?}", user};
    
    Ok(())
}

pub async fn debug_users_show_by_user_id()-> Result<(), Box<dyn std::error::Error>> {
    debug!{"debug users_show_by_user_id"};
    let host: String = get_string_env("DEBUG_MI_HOST", "https://example.tld");
    let user_id = get_string_env("DEBUG_MI_USER_ID", "0000000000000000");
    let user : model::User = endpoint::users::show_by_user_id(&host, &user_id).await?;
    debug!{"{:?}", user};
    
    Ok(())
}

pub async fn debug_users_show_by_username()-> Result<(), Box<dyn std::error::Error>> {
    debug!{"debug users_show_by_username"};
    let host: String = get_string_env("DEBUG_MI_HOST", "https://example.tld");
    let username = get_string_env("DEBUG_MI_USERNAME", "");
    let user : model::User = endpoint::users::show_by_username(&host, &username).await?;
    debug!{"{:?}", user};
    
    Ok(())
}

pub async fn debug_admin_show_moderation_logs()-> Result<(), Box<dyn std::error::Error>> {
    debug!{"debug admin_show_moderation_logs"};
    let host: String = get_string_env("DEBUG_MI_HOST", "https://example.tld");
    let token: String = get_string_env("DEBUG_MI_TOKEN", "XXXXXXXXXXXXXXXX");
    let limit = get_env::<i32>("DEBUG_MI_MODERATION_LOGS_LIMIT", 1);
    let moderation_logs : Vec<model::ModerationLog> = endpoint::admin::show_moderation_logs(&host, &token, limit, None, None).await?;
    debug_check_moderation_logs(&moderation_logs).await?;

    Ok(())
}


pub async fn debug_admin_show_moderation_logs_by_since_id()-> Result<(), Box<dyn std::error::Error>> {
    debug!{"debug admin_show_moderation_logs_by_since_id"};
    let host: String = get_string_env("DEBUG_MI_HOST", "https://example.tld");
    let token: String = get_string_env("DEBUG_MI_TOKEN", "XXXXXXXXXXXXXXXX");
    let limit = get_env::<i32>("DEBUG_MI_MODERATION_LOGS_LIMIT", 1);
    let since_id: String = get_string_env("DEBUG_MI_MODERATION_LOGS_SINCE_ID", "zzzzzzzzzzzzzzzz");
    let moderation_logs : Vec<model::ModerationLog> = endpoint::admin::show_moderation_logs_by_since_id(&host, &token, limit, None, None,&since_id).await?;
    debug_check_moderation_logs(&moderation_logs).await?;
    
    Ok(())
}

pub async fn debug_admin_show_moderation_logs_by_until_id()-> Result<(), Box<dyn std::error::Error>> {
    debug!{"debug admin_show_moderation_logs_by_until_id"};
    let host: String = get_string_env("DEBUG_MI_HOST", "https://example.tld");
    let token: String = get_string_env("DEBUG_MI_TOKEN", "XXXXXXXXXXXXXXXX");
    let until_id = get_string_env("DEBUG_MI_MODERATION_LOGS_UNTIL_ID", "0000000000000000");
    let limit = get_env::<i32>("DEBUG_MI_MODERATION_LOGS_LIMIT", 1);
    let moderation_logs : Vec<model::ModerationLog> = endpoint::admin::show_moderation_logs_by_until_id(&host, &token, limit, None, None, &until_id).await?;
    debug_check_moderation_logs(&moderation_logs).await?;
    
    Ok(())
}

pub async fn debug_admin_show_moderation_logs_by_until_id_since_id()-> Result<(), Box<dyn std::error::Error>> {
    debug!{"debug admin_show_moderation_logs_by_until_id_since_id"};
    let host: String = get_string_env("DEBUG_MI_HOST", "https://example.tld");
    let token: String = get_string_env("DEBUG_MI_TOKEN", "XXXXXXXXXXXXXXXX");
    let since_id: String = get_string_env("DEBUG_MI_MODERATION_LOGS_SINCE_ID", "zzzzzzzzzzzzzzzz");
    let until_id: String = get_string_env("DEBUG_MI_MODERATION_LOGS_UNTIL_ID", "0000000000000000");
    let limit = get_env::<i32>("DEBUG_MI_MODERATION_LOGS_LIMIT", 1);
    let moderation_logs : Vec<model::ModerationLog> = endpoint::admin::show_moderation_logs_by_until_id_since_id(&host, &token, limit, None, None, &until_id, &since_id).await?;
    debug!{"{:?}", moderation_logs};

    debug_check_moderation_logs(&moderation_logs).await?;
    
    Ok(())
}

pub async fn debug_check_moderation_logs(moderation_logs : &Vec<model::ModerationLog>) -> Result<(), Box<dyn std::error::Error>>{

    for log in moderation_logs.iter() {
        let moderation_type = match &log.r#type {
            Some(x) => model::ModerationType::from_string(x),
            None => model::ModerationType::Other,
        };
        debug!{"{:?}", moderation_type};
        match moderation_type {
            model::ModerationType::AddCustomEmoji => {
                match &log.info {
                    Some(x) => {
                        match &x.emoji {
                            Some(y) => {
                                let emoji = y;
                                debug!("{:?}", emoji);
                            },
                            None => (),
                        }
                    },
                    None => (),
                }
            },
            model::ModerationType::UpdateCustomEmoji => {
                match &log.info {
                    Some(x) => {
                        match &x.after {
                            Some(y) => {
                                let emoji = y.to_custom_emoji()?;
                                debug!("{:?}", emoji);
                            },
                            None => (),
                        }
                    },
                    None => (),
                }
            },
            model::ModerationType::DeleteCustomEmoji => {
                match &log.info {
                    Some(x) => {
                        match &x.emoji {
                            Some(y) => {
                                let emoji = y;
                                debug!("{:?}", emoji);
                            },
                            None => (),
                        }
                    },
                    None => (),
                }
            },
            model::ModerationType::CreateAvatarDecoration => {
                match &log.info {
                    Some(x) => {
                        match &x.avatarDecoration {
                            Some(y) => {
                                let deco = y;
                                debug!("{:?}", deco);
                            },
                            None => (),
                        }
                    },
                    None => (),
                }
            },
            model::ModerationType::UpdateAvatarDecoration => {
                match &log.info {
                    Some(x) => {
                        match &x.after {
                            Some(y) => {
                                let deco = y.to_avator_decoration()?;
                                debug!("{:?}", deco);
                            },
                            None => (),
                        }
                    },
                    None => (),
                }
            },
            model::ModerationType::DeleteAvatarDecoration => {
                match &log.info {
                    Some(x) => {
                        match &x.avatarDecoration {
                            Some(y) => {
                                let deco = y;
                                debug!("{:?}", deco);
                            },
                            None => (),
                        }
                    },
                    None => (),
                }
            },
            _ => debug!("type: {:?}", log.r#type.as_ref().unwrap()),
        }
    }
   
   Ok(())
}