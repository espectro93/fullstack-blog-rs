use diesel;
use diesel::prelude::*;
use rocket::{post, State};
use rocket::serde::json::Json;
use rocket::serde::json::serde_json::json;
use serde_json::Value;

use crate::database::DbConn;
use crate::models::user::{NewUser, User};
use crate::responses::{
    conflict, created, internal_server_error, ok, unauthorized, unprocessable_entity, APIResponse,
};
use crate::schema::users;
use crate::schema::users::dsl::*;

/// Log the user in and return a response with an auth token.
///
/// Return UNAUTHORIZED in case the user can't be found or if the password is incorrect.
#[post("/login", data = "<user_in>", format = "application/json")]
pub fn login(
    user_in: Json<String>,
    db: DbConn,
) -> Result<APIResponse, APIResponse> {
    let user_q = users
        .filter(email.eq(&user_in.email))
        .first::<User>(&*db)
        .optional()?;

    // For privacy reasons, we'll not provide the exact reason for failure here (although this
    // could probably be timing attacked to find out whether users exist or not.
    let mut user =
        user_q.ok_or_else(|| unauthorized().message("Username or password incorrect."))?;

    if !user.verify_password(user_in.password.as_str()) {
        return Err(unauthorized().message("Username or password incorrect."));
    }

    Ok(ok().data(json!({
        "user_id": user.id,
        "token": "token",
    })))
}

/// Register a new user using email and password.
///
/// Return CONFLICT is a user with the same email already exists.
#[post("/register", data = "<user>", format = "application/json")]
pub fn register(
    user: Result<UserLogin, Value>,
    db: DbConn,
) -> Result<APIResponse, APIResponse> {
    let user_data = user.map_err(unprocessable_entity)?;

    let new_password_hash = User::make_password_hash(user_data.password.as_str());
    let new_user = NewUser {
        email: user_data.email.clone(),
        password_hash: new_password_hash,
    };

    let insert_result = diesel::insert_into(users::table)
        .values(&new_user)
        .get_result::<User>(&*db);
    if let Err(diesel::result::Error::DatabaseError(
                   diesel::result::DatabaseErrorKind::UniqueViolation,
                   _,
               )) = insert_result
    {
        return Err(conflict().message("User already exists."));
    }

    let user = insert_result?;
    Ok(created().data(json!(&user)))
}