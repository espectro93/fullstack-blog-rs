use std::fmt;
use std::fmt::Formatter;
use chrono::NaiveDateTime;
use rocket::serde::uuid::Uuid;
use rocket::serde::{Serialize, Deserialize};

use crate::models::user::User;
use crate::models::post::Post;

use crate::schema::comments;
use crate::schema::comments::{post_id, user_id};

#[derive(Queryable, Identifiable, Associations)]
#[belongs_to(User)]
#[belongs_to(Post)]
#[table_name = "comments"]
pub struct Comment {
    pub id: Uuid,
    pub user_id: Uuid,
    pub post_id: Uuid,
    pub body: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl fmt::Display for Comment {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{id} (user_id: {user_id}, post_id: {post_id})", id = self.id, user_id = self.user_id, post_id = self.post_id)
    }
}