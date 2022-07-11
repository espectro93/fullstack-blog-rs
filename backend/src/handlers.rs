use rocket::http::Status;
use rocket::request::{self, FromRequest, Request};
use rocket::{catch};
use rocket::outcome::Outcome;

use crate::database::DbConn;

use crate::responses::{APIResponse, BadRequest, Unauthorized, Forbidden, NotFound, InternalServerError, ServiceUnavailable};
use crate::models::user::{User};

#[catch(400)]
pub fn bad_request_handler() -> APIResponse {
    APIResponse::from(BadRequest { data: "Bad Request" })
}

#[catch(401)]
pub fn unauthorized_handler() -> APIResponse {
    APIResponse::from(Unauthorized { data: "Unauthorized" })
}

#[catch(403)]
pub fn forbidden_handler() -> APIResponse {
    APIResponse::from(Forbidden { data: "Forbidden" })
}

#[catch(404)]
pub fn not_found_handler() -> APIResponse {
    APIResponse::from(NotFound { data: "Not found" })
}

#[catch(500)]
pub fn internal_server_error_handler() -> APIResponse {
    APIResponse::from(InternalServerError { data: "Internal server error" })
}

#[catch(503)]
pub fn service_unavailable_handler() -> APIResponse {
    APIResponse::from(ServiceUnavailable { data: "Service unavailable" })
}

impl<'a> FromRequest<'a> for User {
    type Error = ();

    fn from_request(request: &'a Request) -> request::Outcome<User, ()> {
        let db = <DbConn as FromRequest>::from_request(request)?;
        let keys: Vec<_> = request.headers().get("Authorization").collect();
        if keys.len() != 1 {
            return Outcome::Failure((Status::BadRequest, ()));
        };

        let token_header = keys[0];
        let token = token_header.replace("Bearer ", "");

        match User::get_user_from_login_token(&token, &*db) {
            Some(user) => Outcome::Success(user),
            None => Outcome::Failure((Status::Unauthorized, ())),
        }
    }
}