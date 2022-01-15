use std::fmt;
use std::fmt::Formatter;

use chrono::NaiveDateTime;
use rocket::serde::{Serialize, Deserialize};

use crate::models::user::User;

use crate::schema::posts;

#[derive(Debug, Serialize, Deserialize, Queryable, Associations, Identifiable)]
#[belongs_to(User)]
#[table_name = "posts"]
pub struct Post {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub body: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub status: Status,
}

#[derive(Insertable)]
#[table_name="posts"]
pub struct NewPost {
    pub title: String,
    pub body: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Status {
    Draft,
    Published { at: NaiveDateTime },
}

use diesel::pg::Pg;
use diesel::query_source::Queryable;
use diesel::sql_types::{Nullable, Timestamp};
use rocket::serde::uuid::Uuid;

impl Queryable<Nullable<Timestamp>, Pg> for Status {
    type Row = Option<NaiveDateTime>;

    fn build(row: Self::Row) -> Self {
        match row {
            Some(at) => Status::Published { at },
            None => Status::Draft,
        }
    }
}

impl fmt::Display for Post {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{title} (id: {id}). By user_id {user_id}", title = self.title, id = self.id, user_id = self.user_id)
    }
}