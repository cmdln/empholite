mod convert;
mod eval;

use crate::schema::{recipes, rules};
use chrono::NaiveDateTime;
use diesel::{Associations, Identifiable, Insertable, Queryable};
use diesel_derive_enum::DbEnum;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Queryable, Identifiable, Associations)]
pub(crate) struct Recipe {
    pub(crate) id: Uuid,
    pub(crate) url: String,
    pub(crate) payload: String,
    pub(crate) created_at: NaiveDateTime,
    pub(crate) updated_at: NaiveDateTime,
}

pub(crate) struct RecipeCascaded(pub(crate) Recipe, pub(crate) Vec<Rule>);

#[derive(Insertable)]
#[table_name = "recipes"]
pub(crate) struct NewRecipe {
    pub(crate) url: String,
    pub(crate) payload: String,
}

#[derive(DbEnum, Deserialize, Debug)]
pub(crate) enum RuleType {
    Authenticated,
    Subject,
    HttpMethod,
}

#[derive(DbEnum, Deserialize, Debug)]
pub(crate) enum HttpVerb {
    Get,
    Post,
    Put,
    Delete,
}

#[derive(Queryable, Identifiable, Associations, Debug)]
#[belongs_to(Recipe)]
pub(crate) struct Rule {
    pub(crate) id: Uuid,
    pub(crate) recipe_id: Uuid,
    pub(crate) rule_type: RuleType,
    pub(crate) key_path: Option<String>,
    pub(crate) subject: Option<String>,
    pub(crate) http_method: Option<HttpVerb>,
}

#[derive(Insertable)]
#[table_name = "rules"]
pub(crate) struct NewRule {
    pub(crate) recipe_id: Uuid,
    pub(crate) rule_type: RuleType,
    pub(crate) key_path: Option<String>,
    pub(crate) subject: Option<String>,
    pub(crate) http_method: Option<HttpVerb>,
}
