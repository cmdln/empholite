mod convert;

use crate::schema::recipes;
use chrono::NaiveDateTime;
use diesel::{Identifiable, Queryable};
use uuid::Uuid;

// TODO pass timestamps to transfer type
#[allow(unused)]
#[derive(Queryable, Identifiable)]
pub(crate) struct Recipe {
    pub(crate) id: Uuid,
    pub(crate) url: String,
    pub(crate) payload: String,
    pub(crate) created_at: NaiveDateTime,
    pub(crate) updated_at: NaiveDateTime,
}
