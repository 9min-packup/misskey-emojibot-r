use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct EmojibotConfig {
    #[serde(default="Bot::default")]
    pub bot : Bot,
    #[serde(default="Notify::default")]    
    pub notify : Notify,
    #[serde(default="Messages::default")]
    pub messages : Messages,
}

impl Default for EmojibotConfig {
    fn default() -> Self {
        Self {
            bot : Bot::default(),
            notify : Notify::default(),
            messages : Messages::default(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Bot {
    #[serde(default="Bot::defailt_host")]
    pub host : String,
    #[serde(default="Bot::defailt_token")]
    pub token : String,
    #[serde(default="Bot::defailt_moderation_logs_limit")]
    pub moderation_logs_limit : u64,
    #[serde(default="Bot::defailt_running_interval_seconds")]
    pub running_interval_seconds : u64,
}

impl Default for Bot {
    fn default() -> Self {
        Self {
                host : Bot::defailt_host(),
                token : Bot::defailt_token(),
                moderation_logs_limit : Bot::defailt_moderation_logs_limit(),
                running_interval_seconds : Bot::defailt_running_interval_seconds(),
        }
    }
}

impl Bot {
    pub fn defailt_host() -> String { String::from("example.tld")}
    pub fn defailt_token() -> String { String::from("XXXXXXXXXXXXXXXX")}
    pub fn defailt_moderation_logs_limit() -> u64 { 5 }
    pub fn defailt_running_interval_seconds() -> u64 { 60 }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Notify {
    #[serde(default="Visibility::default")]
    pub visibility : Visibility,
    #[serde(default="Notify::default_visible_usernames")]
    pub visible_usernames : Vec<String>,
    #[serde(default="UseCw::default")]
    pub use_cw : UseCw,
    #[serde(default="Notify::default_local_only")]
    pub local_only : bool,
    #[serde(default="Notify::default_reaction_acceptance")]
    pub reaction_acceptance : String,
    #[serde(default="Notify::default_use_mention")]
    pub use_mention : bool,
}

impl Default for Notify {
    fn default() -> Self {
        Self {
            visibility : Visibility::default(),
            visible_usernames : Notify::default_visible_usernames(),
            use_cw : UseCw::default(),
            local_only : Notify::default_local_only(),
            reaction_acceptance : Notify::default_reaction_acceptance(),
            use_mention : Notify::default_use_mention(),
        }
    }
}

impl Notify {
    pub fn default_visible_usernames() -> Vec<String> { Vec::<String>::new() }
    pub fn default_local_only() -> bool { true }
    pub fn default_reaction_acceptance() -> String { String::from("all")  }
    pub fn default_use_mention() -> bool { true }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Visibility {
    #[serde(default="Visibility::default_add")]
    pub add : String,
    #[serde(default="Visibility::default_update")]
    pub update : String,
    #[serde(default="Visibility::default_delete")]
    pub delete : String,
}

impl Default for Visibility {
    fn default() -> Self {
        Self {
            add : Visibility::default_add(),
            update : Visibility::default_update(),
            delete :  Visibility::default_delete(),
        }
    }
}

impl Visibility {
    pub fn default_add() -> String { String::from("public")  }
    pub fn default_update() -> String { String::from("home")  }
    pub fn default_delete() -> String { String::from("home")  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UseCw {
    #[serde(default="UseCw::default_add")]
    pub add : bool,
    #[serde(default="UseCw::default_update")]
    pub update : bool,
    #[serde(default="UseCw::default_delete")]
    pub delete :  bool,
}

impl Default for UseCw {
    fn default() -> Self {
        Self {
            add : UseCw::default_add(),
            update : UseCw::default_update(),
            delete :  UseCw::default_delete(),
        }
    }
}

impl UseCw {
    pub fn default_add() -> bool { true }
    pub fn default_update() -> bool { true }
    pub fn default_delete() -> bool { true }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Messages {
    #[serde(default="Messages::default_emoji_add")]
    pub emoji_add : String,
    #[serde(default="Messages::default_emoji_add_user")]
    pub emoji_add_user : String,
    #[serde(default="Messages::default_emoji_update")]
    pub emoji_update : String,
    #[serde(default="Messages::default_emoji_update_user")]
    pub emoji_update_user : String,
    #[serde(default="Messages::default_emoji_delete")]
    pub emoji_delete : String,
    #[serde(default="Messages::default_emoji_delete_user")]
    pub emoji_delete_user : String,
    #[serde(default="Messages::default_decoration_add")]
    pub decoration_add : String,
    #[serde(default="Messages::default_decoration_add_user")]
    pub decoration_add_user : String,
    #[serde(default="Messages::default_decoration_update")]
    pub decoration_update : String,
    #[serde(default="Messages::default_decoration_update_user")]
    pub decoration_update_user : String,
    #[serde(default="Messages::default_decoration_delete")]
    pub decoration_delete : String,
    #[serde(default="Messages::default_decoration_delete_user")]
    pub decoration_delete_user : String,
}

impl Default for Messages {
    fn default() -> Self {
        Self {
            emoji_add : Messages::default_emoji_add() ,
            emoji_add_user : Messages::default_emoji_add_user() ,
            emoji_update : Messages::default_emoji_update() ,
            emoji_update_user : Messages::default_emoji_update_user() ,
            emoji_delete : Messages::default_emoji_delete() ,
            emoji_delete_user : Messages::default_emoji_delete_user() ,
            decoration_add : Messages::default_decoration_add() ,
            decoration_add_user : Messages::default_decoration_add_user() ,
            decoration_update : Messages::default_decoration_update() ,
            decoration_update_user : Messages::default_decoration_update_user() ,
            decoration_delete : Messages::default_decoration_delete() ,
            decoration_delete_user : Messages::default_decoration_delete_user() ,
        }
    }
}

impl  Messages {
    pub fn default_emoji_add() -> String { String::from("新しい絵文字が追加されました。") }
    pub fn default_emoji_add_user() -> String { String::from("追加したユーザー") } 
    pub fn default_emoji_update() -> String { String::from("絵文字が更新されました。") }
    pub fn default_emoji_update_user() -> String { String::from("更新したユーザー") } 
    pub fn default_emoji_delete() -> String { String::from("絵文字が削除されました。") }
    pub fn default_emoji_delete_user() -> String { String::from("削除したユーザー") } 
    pub fn default_decoration_add() -> String { String::from("新しいアバターデコレーションが追加されました。") }
    pub fn default_decoration_add_user() -> String { String::from("追加したユーザー") } 
    pub fn default_decoration_update() -> String { String::from("アバターデコレーションが更新されました。") }
    pub fn default_decoration_update_user() -> String { String::from("更新したユーザー") } 
    pub fn default_decoration_delete() -> String { String::from("アバターデコレーションが削除されました。") }
    pub fn default_decoration_delete_user() -> String { String::from("削除したユーザー") } 

}

