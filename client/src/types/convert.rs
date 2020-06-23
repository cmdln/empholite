use super::Recipe;

impl From<shared::Recipe> for Recipe {
    fn from(r: shared::Recipe) -> Self {
        let shared::Recipe {
            id,
            url,
            payload,
            created_at,
            updated_at,
        } = r;
        let rules = Vec::new();
        Self {
            id,
            url,
            rules,
            payload,
            created_at,
            updated_at,
        }
    }
}
