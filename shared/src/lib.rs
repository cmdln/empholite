use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct Recipe {
    pub url: String,
    pub payload: String,
}
