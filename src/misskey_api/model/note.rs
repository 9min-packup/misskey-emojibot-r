use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use super::{User, File};

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Note {
	id : String,
	createdAt : String,
	userId : String,
	user : User,
	text : String,
	cw : Option<String>,
	visibility : String,
	localOnly : bool,
	reactionAcceptance : Option<String>,
	renoteCount : i32,
	repliesCount : i32,
	reactions : HashMap<String, i32>,
	reactionEmojis : HashMap<String, i32>,
	fileIds : Vec<String>,
	files : Vec<File>,
	replyId : Option<String>,
	renoteId : Option<String>,
	clippedCount : i32,
}	