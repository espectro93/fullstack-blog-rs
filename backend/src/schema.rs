table! {
    comments (id) {
        id -> Uuid,
        user_id -> Uuid,
        post_id -> Uuid,
        body -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    posts (id) {
        id -> Uuid,
        user_id -> Uuid,
        title -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        body -> Text,
        status -> Text,
        published_at -> Nullable<Timestamp>,
    }
}

table! {
    users (id) {
        id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        email -> Varchar,
        password_hash -> Bytea,
        last_action -> Nullable<Timestamp>,
    }
}

joinable!(comments -> posts (post_id));
joinable!(comments -> users (user_id));
joinable!(posts -> users (user_id));

allow_tables_to_appear_in_same_query!(
    comments,
    posts,
    users,
);
