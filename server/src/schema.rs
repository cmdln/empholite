table! {
    recipes (id) {
        id -> Uuid,
        url -> Varchar,
        payload -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    use diesel::{sql_types::{Uuid, Nullable}, types::Varchar};
    use crate::models::RuleTypeMapping;

    rules (id) {
        id -> Uuid,
        recipe_id -> Uuid,
        rule_type -> RuleTypeMapping,
        key_path -> Nullable<Varchar>,
        subject -> Nullable<Varchar>,
    }
}

joinable!(rules -> recipes (recipe_id));

allow_tables_to_appear_in_same_query!(recipes, rules,);
