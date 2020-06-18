use super::Recipe;

impl Into<shared::Recipe> for Recipe {
    fn into(self) -> shared::Recipe {
        let Recipe {
            url, payload, id, ..
        } = self;
        let id = Some(id);

        shared::Recipe { id, url, payload }
    }
}
