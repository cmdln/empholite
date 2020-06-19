use super::Recipe;

impl Into<shared::Recipe> for Recipe {
    fn into(self) -> shared::Recipe {
        let Recipe {
            url,
            payload,
            id,
            created_at,
            updated_at,
        } = self;
        let id = Some(id);
        let created_at = Some(created_at);
        let updated_at = Some(updated_at);
        shared::Recipe {
            id,
            url,
            payload,
            created_at,
            updated_at,
        }
    }
}
