use std::f32::consts::E;

use serde::{Deserialize, Serialize};
use super::{User, CustmoEmoji, AvatorDecoration};

#[derive(Debug, Serialize, Deserialize)]
pub struct ModerationLog {
    pub id : String,
    pub createdAt : String,
    pub r#type : Option<String>,
    pub info : Option<ModerationLogInfo>,
    pub userId : String,
    pub user : User,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModerationLogInfo {
    pub emoji : Option<CustmoEmoji>,
    pub avatarDecoration : Option<AvatorDecoration>,
    pub before : Option<ModerationLogInfoForUpdate>,
    pub after : Option<ModerationLogInfoForUpdate>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModerationLogInfoForUpdate {
    pub id : Option<String>,
    pub aliases : Option<Vec<String>>,
    pub name : Option<String>,
    pub url : Option<String>,
    pub description : Option<String>,
    pub category : Option<String>,
    pub host : Option<String>, 
    pub publicUrl : Option<String>,
    pub originalUrl : Option<String>,
    pub license : Option<String>,   
    pub isSensitive : Option<bool>,
    pub localOnly : Option<bool>,
    pub roleIdsThatCanBeUsedThisEmojiAsReaction : Option<Vec<Option<String>>>,
}

#[derive(Debug)]
pub struct NotCustomEmoji{
    invalid_field : String,
}

impl std::error::Error for NotCustomEmoji {}

impl std::fmt::Display for NotCustomEmoji {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Not CustomEmoji. field {} is invalid.", self.invalid_field)
    }
}

impl ModerationLogInfoForUpdate {
    pub fn to_custom_emoji(&self) -> Result<CustmoEmoji, Box<dyn std::error::Error>> {
        let id: String = match &self.id {
            Some(x) => x.clone(),
            None => return Err( Box::new(NotCustomEmoji{invalid_field : String::from("id")})),
        };
        let aliases: Vec<String> = match &self.aliases {
            Some(x) => x.clone(),
            None => return Err( Box::new(NotCustomEmoji{invalid_field : String::from("aliases")})),
        };
        let name: String = match &self.name {
            Some(x) => x.clone(),
            None => return Err( Box::new(NotCustomEmoji{invalid_field : String::from("name")})),
        };
        let isSensitive: bool = match &self.isSensitive {
            Some(x) => x.clone(),
            None => return Err( Box::new(NotCustomEmoji{invalid_field : String::from("isSensitive")})),
        };
        let localOnly: bool = match &self.localOnly {
            Some(x) => x.clone(),
            None => return Err( Box::new(NotCustomEmoji{invalid_field : String::from("localOnly")})),
        };
        let roleIdsThatCanBeUsedThisEmojiAsReaction: Vec<Option<String>> = match &self.roleIdsThatCanBeUsedThisEmojiAsReaction {
            Some(x) => x.clone(),
            None => return Err( Box::new(NotCustomEmoji{invalid_field : String::from("roleIdsThatCanBeUsedThisEmojiAsReaction")})),
        };

        let category = self.category.clone();
        let host = self.host.clone();
        let publicUrl = self.publicUrl.clone();
        let originalUrl = self.originalUrl.clone();
        let license = self.license.clone();
                    
        Ok(CustmoEmoji {
            id : id,
            aliases : aliases,
            name : name,
            category : category,
            host : host, 
            publicUrl : publicUrl,
            originalUrl : originalUrl,
            license : license,   
            isSensitive : isSensitive,
            localOnly : localOnly,
            roleIdsThatCanBeUsedThisEmojiAsReaction : roleIdsThatCanBeUsedThisEmojiAsReaction,
        })
    }
}

#[derive(Debug)]
pub struct NotAvatorDecoration{
    invalid_field : String,
}

impl std::error::Error for NotAvatorDecoration {}

impl std::fmt::Display for NotAvatorDecoration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Not AvatorDecoration. field {} is invalid.", self.invalid_field)
    }
}


impl ModerationLogInfoForUpdate {
    pub fn to_avator_decoration(&self) -> Result<AvatorDecoration, Box<dyn std::error::Error>> {
        let id: String = match &self.id {
            Some(x) => x.clone(),
            None => return Err( Box::new(NotAvatorDecoration{invalid_field : String::from("id")})),
        };
        let url: String = match &self.url {
            Some(x) => x.clone(),
            None => return Err( Box::new(NotAvatorDecoration{invalid_field : String::from("url")})),
        };
        let name: String = match &self.name {
            Some(x) => x.clone(),
            None => return Err( Box::new(NotAvatorDecoration{invalid_field : String::from("name")})),
        };

        let description = self.description.clone();
                    
        Ok(AvatorDecoration {
            id : id,
            url : url,
            name : name,
            description : description,
        })
    }
}