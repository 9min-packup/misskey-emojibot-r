use serde::{Deserialize, Serialize};
use super::User;

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct File {
    pub id : String,
    pub createdAt : String,
    pub name : String,
    pub r#type : String,
    pub md5 : String,
    pub size : i32,
    pub isSensitive : bool,
    pub blurhash : Option<String>,
    pub properties : FileProperties,
    pub url : Option<String>,
    pub thumbnailUrl : Option<String>,
    pub comment : Option<String>,
	pub folderId : Option<String>,
	pub userId : Option<String>,
	pub user : Option<User>,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct FileProperties {
    pub width : Option<i32>,
    pub height : Option<i32>,
}
