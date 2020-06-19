mod convert;

use crate::schema::recipes;
use chrono::NaiveDateTime;
use diesel::{Identifiable, Insertable, Queryable};
use uuid::Uuid;

#[derive(Queryable, Identifiable)]
pub(crate) struct Recipe {
    pub(crate) id: Uuid,
    pub(crate) url: String,
    pub(crate) payload: String,
    pub(crate) created_at: NaiveDateTime,
    pub(crate) updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "recipes"]
pub(crate) struct NewRecipe {
    pub(crate) url: String,
    pub(crate) payload: String,
}
