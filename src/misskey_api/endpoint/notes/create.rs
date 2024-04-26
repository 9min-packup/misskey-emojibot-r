use serde::Serialize;
use super::{Visibility, ReactionAcceptance};
use super::super::{request_json, ApiTarget};
use super::super::super::model::CreatedNote;

#[allow(non_snake_case)]
#[derive(Debug, Serialize)]
struct Param{
    i : String,
    text : String,
    cw : Option<String>,
    visibility : String,
    visibleUserIds : Vec<String>,
    localOnly : bool,
    reactionAcceptance : Option<String>,
}

pub async fn create
(
    host : &str,
    token : &str,
    text : &str,
    cw : Option<String>,
    visibility : Visibility,
    visible_user_ids : Vec<String>,
    local_only : bool,
    reaction_acceptance : ReactionAcceptance
) -> Result<CreatedNote, Box<dyn std::error::Error>> {
    let param = Param{
        i : String::from(token),
        text : String::from(text),
        cw : cw,
        visibility : Visibility::to_string(&visibility),
        visibleUserIds : visible_user_ids.clone(),
        localOnly : local_only,
        reactionAcceptance : ReactionAcceptance::to_string(&reaction_acceptance),
    };

    let created_note = request_json::<Param, CreatedNote>(&host, ApiTarget::NotesCreate, &param).await?;

    Ok(created_note)
}
