use serde::{Deserialize, Serialize};
use super::super::model::User;
use super::request;

#[derive(Serialize, Debug)]
pub struct Param {
    id : String,
}
