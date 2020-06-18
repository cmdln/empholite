mod convert;

use chrono::NaiveDateTime;
use uuid::Uuid;

// TODO pass timestamps to transfer type
#[allow(unused)]
#[derive(Queryable)]
pub(crate) struct Recipe {
    pub(crate) id: Uuid,
    pub(crate) url: String,
    pub(crate) payload: String,
    pub(crate) created_at: NaiveDateTime,
    pub(crate) updated_at: NaiveDateTime,
}
