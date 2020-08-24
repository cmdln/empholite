use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct Recipe {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Uuid>,
    pub url: String,
    #[serde(default = "Vec::new")]
    pub rules: Vec<Rule>,
    pub payload: Value,
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
    HttpMethod {
        #[serde(skip_serializing_if = "Option::is_none")]
        id: Option<Uuid>,
        http_method: HttpVerb,
    },
}

#[derive(Serialize, Deserialize, Debug)]
pub enum HttpVerb {
    Get,
    Post,
    Put,
    Delete,
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
