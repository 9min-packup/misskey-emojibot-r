use std::env;
use log::{trace, debug};
use serde::Serialize;
use crate::misskey_api::endpoint;
use crate::misskey_api::model;

#[derive(Debug, Serialize)]
struct Param{
    i : String,
}

pub async fn demo() -> Result<(), Box<dyn std::error::Error>>  {
    //demo_request().await?;
    //demo_i().await?;
    //demo_users_show_by_userid().await?;
    //demo_users_show_by_username().await?;
    demo_admin_show_moderation_logs().await?;
    //demo_admin_show_moderation_logs_by_since_id().await?;
    //demo_admin_show_moderation_logs_by_until_id().await?;

    Ok(())
}

pub async fn demo_request()-> Result<(), Box<dyn std::error::Error>> {
    debug!{"demo request"};
    let host: String = env::var("HOST").expect("HOST is not found in .env");
    let token: String = env::var("TOKEN").expect("TOKEN is not found in .env");
    let param: Param = Param{i : token};
    let user : model::User = endpoint::request::<Param, model::User>(&host, endpoint::ApiTarget::I, &param).await?;

    debug!{"{:?}", user};

    Ok(())
}

pub async fn demo_i()-> Result<(), Box<dyn std::error::Error>> {
    debug!{"demo i"};
    let host: String = env::var("HOST").expect("HOST is not found in .env");
    let token: String = env::var("TOKEN").expect("TOKEN is not found in .env");
    let user : model::User = endpoint::i::i(&host, &token).await?;
    debug!{"{:?}", user};
    
    Ok(())
}

pub async fn demo_users_show_by_userid()-> Result<(), Box<dyn std::error::Error>> {
    debug!{"demo users_show_by_userid"};
    let host: String = env::var("HOST").expect("HOST is not found in .env");
    let user : model::User = endpoint::users::show_by_user_id(&host, "9sh25s64bb9l000e").await?;
    debug!{"{:?}", user};
    
    Ok(())
}

pub async fn demo_users_show_by_username()-> Result<(), Box<dyn std::error::Error>> {
    debug!{"demo users_show_by_username"};
    let host: String = env::var("HOST").expect("HOST is not found in .env");
    let user : model::User = endpoint::users::show_by_username(&host, "emojibot").await?;
    debug!{"{:?}", user};
    
    Ok(())
}

pub async fn demo_admin_show_moderation_logs()-> Result<(), Box<dyn std::error::Error>> {
    debug!{"demo admin_show_moderation_logs"};
    let host: String = env::var("HOST").expect("HOST is not found in .env");
    let token: String = env::var("TOKEN").expect("TOKEN is not found in .env");
    let limit = 5;
    let moderation_logs : Vec<model::ModerationLog> = endpoint::admin::show_moderation_logs(&host, &token, limit, None).await?;
    debug!{"{:?}", moderation_logs};

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
            _ => (),
        }
    }

    Ok(())
}


pub async fn demo_admin_show_moderation_logs_by_since_id()-> Result<(), Box<dyn std::error::Error>> {
    debug!{"demo admin_show_moderation_logs_by_since_id"};
    let host: String = env::var("HOST").expect("HOST is not found in .env");
    let token: String = env::var("TOKEN").expect("TOKEN is not found in .env");
    let limit = 5;
    let since_id: String = String::from("9sh2xc21bb9l001e");
    let moderation_logs : Vec<model::ModerationLog> = endpoint::admin::show_moderation_logs_by_since_id(&host, &token, limit, None, &since_id).await?;
    debug!{"{:?}", moderation_logs};
    
    Ok(())
}

pub async fn demo_admin_show_moderation_logs_by_until_id()-> Result<(), Box<dyn std::error::Error>> {
    debug!{"demo admin_show_moderation_logs_by_until_id"};
    let host: String = env::var("HOST").expect("HOST is not found in .env");
    let token: String = env::var("TOKEN").expect("TOKEN is not found in .env");
    let until_id: String = String::from("9sh27rggbb9l000n");
    let limit = 5;
    let moderation_logs : Vec<model::ModerationLog> = endpoint::admin::show_moderation_logs_by_until_id(&host, &token, limit, None, &until_id).await?;
    debug!{"{:?}", moderation_logs};
    
    Ok(())
}