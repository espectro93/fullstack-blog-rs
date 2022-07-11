use diesel;
use diesel::prelude::*;
use rocket::{post, State};
use rocket::serde::json::Json;
use rocket::serde::json::serde_json::json;
use serde_json::Value;

use crate::database::DbConn;
use crate::models::user::{NewUser, User};
use crate::schema::users;
use crate::schema::users::dsl::*;
use anyhow::Result;
use crate::responses::{Unauthorized};
use crate::api::auth::dto::LoginUser;

pub fn login(login_user: LoginUser, db: DbConn) -> Result<User> {
    let user_q = users
        .filter(email.eq(login_user.username))
        .first::<User>(&*db)
        .optional()?;

    // For privacy reasons, we'll not provide the exact reason for failure here (although this
    // could probably be timing attacked to find out whether users exist or not.
    let mut user =
        user_q.ok_or_else().context("Username or password incorrect.")?;

    if !user.verify_password(login_user.password.as_str()) {
        panic!("Username or password incorrect")
    }

    Ok(user)
}