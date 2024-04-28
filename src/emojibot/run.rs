
use tokio::time::{sleep, Duration};
use std::env;
use log::{error, info, debug};

use crate::util::toml::read_toml;
use crate::misskey_api::endpoint;
use crate::misskey_api::endpoint::notes::{Visibility, ReactionAcceptance};
use crate::misskey_api::model::{AvatorDecoration, CustmoEmoji, ModerationLog, ModerationType};
use super::config::EmojibotConfig;

const TEXT_LEN_LIMIT: usize = 512;

enum NoteType {
    Add,
    Update,
    Delete,
}

pub async fn run() -> Result<EmojibotConfig, Box<dyn std::error::Error>> {
    info!("emojibot start");
    let config_path = match env::var("CONFIG_FILE_PATH") {
        Ok(s) => s,
        Err(e) => {
            error!("CONFIG_FILE_PATH is not found in .env");
            return Err(Box::new(e));
        },
    };
    let config: EmojibotConfig = match read_config(&config_path).await {
        Ok(c) => c,
        Err(e) => {
            error!("{} : no such file", config_path);
            return Err(e);
        },
    };

    let user = match endpoint::i::i(&config.bot.host, &config.bot.token).await {
        Ok(u) => u,
        Err(e) => {
            error!("request failed - i");
            return Err(e);
        },
    };
    let bot_user_id = user.id;

    let mut visible_user_ids = Vec::<String>::new();
    for username in &config.notify.visible_usernames {
        let user = match endpoint::users::show_by_username(&config.bot.host, &username).await {
            Ok (u) => u,
            Err(e) =>{
                error!("request failed - users/show");
                return Err(e);
            }
        };
        visible_user_ids.push(user.id);
    }

    let latest_moderation_logs = 
    match endpoint::admin::show_moderation_logs(
        &config.bot.host,
        &config.bot.token,
        1, 
        None, 
        None
    ).await {
        Ok(u) => u,
        Err(e) => {
            error!("request failed - admin/show_moderation_logs");
            return Err(e);
        },
    };
    let mut since_id :Option<String> = match latest_moderation_logs.len() {
        0 => None,
        _ => Some(String::from(&latest_moderation_logs.get(0).unwrap().id)),
    };

    let interval = config.bot.running_interval_seconds;

    info!("initialize success");

    loop {
        sleep(Duration::from_secs(interval)).await;
        since_id = update(&config, &bot_user_id, &visible_user_ids, &since_id).await;
    }
}

async fn read_config(path : &str) -> Result<EmojibotConfig, Box<dyn std::error::Error>> {
    let config = match read_toml::<EmojibotConfig>(path) {
        Ok(c) => c,
        Err(e) => return Err(e),
    };
    debug!("{:?}", config);

    Ok(config)
}

async fn update(config : &EmojibotConfig, bot_user_id : &str, visible_user_ids : &Vec<String>, since_id : &Option<String>) -> Option<String> {
    let host = &config.bot.host;
    let token = &config.bot.token;
    let limit = config.bot.moderation_logs_limit;
    let r#type = None;
    let user_id = None;
    let mut moderation_logs: Vec<ModerationLog>; 
    // let ml_itor : Iter<ModerationLog>;
    if since_id.is_some() {
        moderation_logs = match endpoint::admin::show_moderation_logs_by_since_id(&host, &token, limit, r#type, user_id, since_id.clone().unwrap().as_str()).await {
            Ok(mls) => mls,
            Err(_e) => {
                error!("request failed - admin/show_moderation_logs");
                debug!("{:?}", since_id);
                return since_id.clone();
            }
        };
    } else {
        moderation_logs = match endpoint::admin::show_moderation_logs(&host, &token, limit, r#type, user_id).await {
            Ok(mls) => mls,
                Err(_e) => {
                    error!("request failed - admin/show_moderation_logs");
                    debug!("{:?}", since_id);
                    return since_id.clone();
                }
        };
        moderation_logs.reserve(moderation_logs.len());
    }
    if moderation_logs.len() == 0 {
        info!("no moderarion log");
        return since_id.clone()
    }

    let ml_id = match moderation_logs.last() {
        Some(m) => Some(String::from(&m.id)),
        None => since_id.clone(),
    };

    for ml in moderation_logs {
        match &ml.r#type {
            Some(s) => info!("moderation log type : {}", s),
            None => continue,
        }
        if &ml.user.id == bot_user_id {
            continue;
        }

        match ModerationType::from_str(&ml.r#type.unwrap()) {
            ModerationType::AddCustomEmoji => {
                let emoji = match ml.info {
                    Some(info) => match info.emoji {
                        Some(emoji) => emoji,
                        None => continue,
                    },
                    None => continue,
                };
                create_note_emoji(NoteType::Add, emoji, ml.user.username, visible_user_ids, config).await;
            },
            ModerationType::UpdateCustomEmoji => {
                let emoji = match ml.info {
                    Some(info) => match info.after {
                        Some(a) => match a.to_custom_emoji() {
                            Ok(emoji) => emoji,
                            Err(e) => {
                                error!("{:?}", e);
                                continue;
                            }
                        },
                        None => continue,
                    }
                    None => continue,
                };
                create_note_emoji(NoteType::Update, emoji, ml.user.username, visible_user_ids, config).await;
            },
            ModerationType::DeleteCustomEmoji => {
                let emoji = match ml.info {
                    Some(info) => match info.emoji {
                        Some(emoji) => emoji,
                        None => continue,
                    },
                    None => continue,
                };
                create_note_emoji(NoteType::Delete, emoji, ml.user.username, visible_user_ids, config).await;
            },
            ModerationType::CreateAvatarDecoration => {
                let deco = match ml.info {
                    Some(info) => match info.avatarDecoration {
                        Some(deco) => deco,
                        None => continue,
                    },
                    None => continue,
                };
                create_note_decoration(NoteType::Add, deco, ml.user.username, visible_user_ids, config).await;
            },
            ModerationType::UpdateAvatarDecoration => {
                let deco = match ml.info {
                    Some(info) => match info.after {
                        Some(a) => match a.to_avator_decoration() {
                            Ok(deco) => deco,
                            Err(e) => {
                                error!("{:?}", e);
                                continue;
                            }
                        },
                        None => continue,
                    }
                    None => continue,
                };
                create_note_decoration(NoteType::Update, deco, ml.user.username, visible_user_ids, config).await;
            },
            ModerationType::DeleteAvatarDecoration => {
                let deco = match ml.info {
                    Some(info) => match info.avatarDecoration {
                        Some(deco) => deco,
                        None => continue,
                    },
                    None => continue,
                };
                create_note_decoration(NoteType::Delete, deco, ml.user.username, visible_user_ids, config).await;
            },
            ModerationType::Other => info!("skip"),
        }
    }

    ml_id

}

async fn create_note_emoji(note_type : NoteType, emoji : CustmoEmoji, username : String,  visible_user_ids : &Vec<String>, config : &EmojibotConfig) {
    let aliases_full = emoji.aliases.join(", ");
    let aliases = if aliases_full.len() > TEXT_LEN_LIMIT {
        substring(&aliases_full, 0, TEXT_LEN_LIMIT) + "..."
    } else {
        aliases_full
    };
    let license : String = match emoji.license {
        Some(s) => if s.len() > TEXT_LEN_LIMIT {
            substring(&s, 0, TEXT_LEN_LIMIT) + "..."
        } else {
            s
        }
        None => String::from("null"),
    };
    let category : String = match emoji.category {
        Some(s) => s,
        None => String::from("Other"),
    };
    let msg = match note_type {
        NoteType::Add => &config.messages.emoji_add,
        NoteType::Update => &config.messages.emoji_update,
        NoteType::Delete => &config.messages.emoji_delete,
    };
    let msg_user = match note_type {
        NoteType::Add => &config.messages.emoji_add_user,
        NoteType::Update => &config.messages.emoji_update_user,
        NoteType::Delete => &config.messages.emoji_delete_user,
    };
    let visibility = match note_type {
        NoteType::Add => &config.notify.visibility.add,
        NoteType::Update => &config.notify.visibility.update,
        NoteType::Delete => &config.notify.visibility.delete,
    };
    let use_cw = match note_type {
        NoteType::Add => &config.notify.use_cw.add,
        NoteType::Update => &config.notify.use_cw.update,
        NoteType::Delete => &config.notify.use_cw.delete,
    };
    let cw_text : Option<String> = match use_cw {
        true => Some(format!("{} :{}:", msg, emoji.name)),
        false => None,
    };
    let header_mention = match Visibility::from_str(&visibility) {
        Visibility::Specified => format!("@{}\n", config.notify.visible_usernames.join(" @")),
        _ => String::from(""),
    };
    let header = match use_cw {
        true => String::from(""),
        false => format!("{} :{}:\n", msg, emoji.name),
    };
    let footer = match config.notify.use_mention {
        true => format!("{}:@{}\n", msg_user, username),
        false => format!("{}: `@{}`\n", msg_user, username),
    };
    let text = match note_type {
        NoteType::Delete => format!("{}{}<small>\nname        : `:{}:`\n{}\n</small>",
            header_mention,
            header,
            emoji.name,
            footer),
        _ => format!("{}{}<small>\nname        : `:{}:`\ncategory    : `{}`\ntags        : `[{}]`\nlicense     : `{}`\nisSensitive : `{}`\nlocalOnly   : `{}`\n{}\n</small>",
            header_mention,
            header,
            emoji.name,
            category,
            aliases,
            license,
            emoji.isSensitive,
            emoji.localOnly,
            footer),
    };

    match endpoint::notes::create(&config.bot.host, &config.bot.token, &text, cw_text, Visibility::from_str(&visibility), visible_user_ids.clone(), config.notify.local_only, ReactionAcceptance::from_str(&config.notify.reaction_acceptance)).await {
        Ok(_cn) => info!("request success - notes/create"),
        Err(_e) => error!("request failed - notes/create"),
    }
}

async fn create_note_decoration(note_type : NoteType, deco : AvatorDecoration, username : String,  visible_user_ids : &Vec<String>,config : &EmojibotConfig) {
    let description : String = match deco.description {
        Some(s) => if s.len() > TEXT_LEN_LIMIT {
            substring(&s, 0, TEXT_LEN_LIMIT) + "..."
        } else {
            s
        }
        None => String::from(""),
    };
    let msg = match note_type {
        NoteType::Add => &config.messages.decoration_add,
        NoteType::Update => &config.messages.decoration_update,
        NoteType::Delete => &config.messages.decoration_delete,
    };
    let msg_user = match note_type {
        NoteType::Add => &config.messages.decoration_add_user,
        NoteType::Update => &config.messages.decoration_update_user,
        NoteType::Delete => &config.messages.decoration_delete_user,
    };
    let visibility = match note_type {
        NoteType::Add => &config.notify.visibility.add,
        NoteType::Update => &config.notify.visibility.update,
        NoteType::Delete => &config.notify.visibility.delete,
    };
    let use_cw = match note_type {
        NoteType::Add => config.notify.use_cw.add,
        NoteType::Update => config.notify.use_cw.update,
        NoteType::Delete => config.notify.use_cw.delete,
    };
    let cw_text : Option<String> = match use_cw {
        true => Some(format!("{} `{}`", msg, deco.name)),
        false => None,
    };
    let header_mention = match Visibility::from_str(&visibility) {
        Visibility::Specified => format!("@{}\n", config.notify.visible_usernames.join(" @")),
        _ => String::from(""),
    };
    let header = match use_cw {
        true => String::from(""),
        false => format!("{} `{}`", msg, deco.name),
    };
    let footer = match config.notify.use_mention {
        true => format!("{}:@{}\n", msg_user, username),
        false => format!("{}: `@{}`\n", msg_user, username),
    };
    let text = match note_type {
    NoteType::Delete =>  format!("{}{}<small>\n name        : `{}`\n{}\n</small>",
        header_mention,
        header,
        deco.name,
        footer),
    _ => format!("{}{}<small>\n name        : `{}`\nurl         : {}\ndescription : `{}`\n{}\n</small>",
        header_mention,
        header,
        deco.name,
        deco.url,
        description,
        footer),
    };

    match endpoint::notes::create(&config.bot.host, &config.bot.token, &text, cw_text, Visibility::from_str(&visibility), visible_user_ids.clone(), config.notify.local_only, ReactionAcceptance::from_str(&config.notify.reaction_acceptance)).await {
        Ok(_cn) => info!("request success - notes/create"),
        Err(_e) => error!("request failed - notes/create"),
    }
}


fn substring(s: &str, start: usize, length: usize) -> String {
    let mut char_indices = s.char_indices();
    let byte_start = match char_indices.nth(start) {
        Some(x) => x.0,
        None => return String::from(""),
    };

    match char_indices.nth(length - 1) {
        Some(x) => String::from(&s[byte_start..x.0]),
        None => String::from(&s[byte_start..],)
    }
}