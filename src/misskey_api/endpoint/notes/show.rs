use serde::Serialize;
use super::super::{request_json, ApiTarget};
use super::super::super::model::Note;

#[allow(non_snake_case)]
#[derive(Debug, Serialize)]
struct Param {
    noteId : String
}

#[allow(dead_code)]
pub async fn show(host : &str, note_id : &str)-> Result<Note, Box<dyn std::error::Error>> {
    let param : Param = Param{noteId : String::from(note_id)};
    let note : Note = request_json::<Param, Note>(&host, ApiTarget::NotesShow, &param).await?;

    Ok(note)
}