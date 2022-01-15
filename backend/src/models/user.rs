use std::fmt;

use argon2rs::argon2i_simple;
use chrono::{Duration, NaiveDateTime, Utc};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use rand::distributions::Alphanumeric;
use rand::{Rng, thread_rng};
use ring::constant_time::verify_slices_are_equal;
use rocket::serde::uuid::Uuid;
use rocket::serde::{Serialize, Deserialize};

use crate::schema::users;

#[derive(Debug, Serialize, Deserialize, Queryable, Identifiable, AsChangeset)]
#[table_name = "users"]
pub struct User{
    pub id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub email: String,
    pub password_hash: Vec<u8>,
    pub last_action: Option<NaiveDateTime>,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub email: String,
    pub password_hash: Vec<u8>,
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<User {email}>", email = self.email)
    }
}

impl User {
    /// Hash `password` using argon2 and return it.
    pub fn make_password_hash(password: &str) -> Vec<u8> {
        argon2i_simple(password, "loginsalt").to_vec()
    }

    /// Verify that `candidate_password` matches the stored password.
    pub fn verify_password(&self, candidate_password: &str) -> bool {
        let candidate_hash = argon2i_simple(candidate_password, "loginsalt").to_vec();
        self.password_hash == candidate_hash
    }
}