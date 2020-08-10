use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct Recipe {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Uuid>,
    pub url: String,
    pub rules: Vec<Rule>,
    pub payload: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<NaiveDateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Rule {
    Authenticated {
        #[serde(skip_serializing_if = "Option::is_none")]
        id: Option<Uuid>,
        key_path: String,
    },
    Subject {
        #[serde(skip_serializing_if = "Option::is_none")]
        id: Option<Uuid>,
        subject: String,
    },
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
