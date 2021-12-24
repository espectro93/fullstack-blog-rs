#![allow(proc_macro_derive_resolution_fallback)]

use diesel::table;
table! {
    users (id) {
        id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        email -> Varchar,
        password_hash -> Bytea,
        current_auth_token -> Nullable<Varchar>,
        last_action -> Nullable<Timestamp>,
    }
}