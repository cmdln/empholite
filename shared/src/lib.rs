use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct Recipe {
    pub id: Option<Uuid>,
    pub url: String,
    pub payload: String,
}
