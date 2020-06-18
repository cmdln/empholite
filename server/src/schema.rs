table! {
    recipes (id) {
        id -> Uuid,
        url -> Varchar,
        payload -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
