// @generated automatically by Diesel CLI.

diesel::table! {
    posts (id) {
        id -> Int4,
        content -> Varchar,
        publish_date -> Timestamptz,
        author -> Nullable<Int4>,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 16]
        username -> Varchar,
        password -> Varchar,
    }
}

diesel::joinable!(posts -> users (author));

diesel::allow_tables_to_appear_in_same_query!(
    posts,
    users,
);
