use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AvatorDecoration {
    pub id : String,
    pub url : String,
    pub name : String,
    pub description : Option<String>,
}
