use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct CustmoEmoji {
    pub id : String,
    pub aliases : Vec<String>,
    pub name : String,
    pub category : Option<String>,
    pub host : Option<String>, 
    pub publicUrl : Option<String>,
    pub originalUrl : Option<String>,
    pub license : Option<String>,   
    pub isSensitive : bool,
    pub localOnly : bool,
    pub roleIdsThatCanBeUsedThisEmojiAsReaction : Vec<Option<String>>,
}
