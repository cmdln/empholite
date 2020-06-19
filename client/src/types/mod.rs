mod convert;

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Default, Serialize, Deserialize, Debug)]
pub(crate) struct Recipe {
    pub(crate) id: Option<Uuid>,
    pub(crate) url: String,
    pub(crate) payload: String,
    pub(crate) created_at: Option<NaiveDateTime>,
    pub(crate) updated_at: Option<NaiveDateTime>,
}
