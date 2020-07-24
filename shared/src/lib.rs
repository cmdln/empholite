use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct Recipe {
    pub id: Option<Uuid>,
    pub url: String,
    pub rules: Vec<Rule>,
    pub payload: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Rule {
    Authenticated { id: Option<Uuid>, key_path: String },
    Subject { id: Option<Uuid>, subject: String },
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum KeyPathKind {
    Directory,
    File,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub key_path_kind: KeyPathKind,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            key_path_kind: KeyPathKind::Directory,
        }
    }
}
