use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize, Debug)]
pub(crate) struct RecipesPage {
    pub(crate) total: i64,
    pub(crate) offset: i64,
    pub(crate) limit: i64,
    pub(crate) recipes: Vec<shared::Recipe>,
}
