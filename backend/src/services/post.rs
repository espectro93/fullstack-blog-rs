use diesel::prelude::*;
use crate::database::DbConn;
use crate::models::post::{NewPost, Post};

use crate::schema::posts;
use crate::schema::posts::dsl::*;

pub fn create_post<'a>(conn: &DbConn, title: &'a str, body: &'a str) -> Post {

    let new_post = NewPost {
        title,
        body,
    };

    diesel::insert_into(posts::table)
        .values(&new_post)
        .get_result(conn)
        .expect("Error saving new post")
}