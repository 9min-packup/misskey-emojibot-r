use serde::{Deserialize, Serialize};
use super::Note;

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct CreatedNote {
    pub createdNote: Note,
}