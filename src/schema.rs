// @generated automatically by Diesel CLI.

diesel::table! {
    todo_items (id) {
        id -> Int4,
        item -> Text,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 20]
        username -> Varchar,
        #[max_length = 64]
        password -> Varchar,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    todo_items,
    users,
);
