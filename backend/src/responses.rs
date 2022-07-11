use diesel::result::Error as DieselError;
use rocket::http::{ContentType, Status};
use rocket::request::Request;
use rocket::response::{Responder, Response};
use std::convert::From;
use std::io::Cursor;
use rocket::serde::json::Json;
use rocket::serde::json::serde_json::json;
use serde_json::Value;

#[derive(Debug)]
pub struct APIResponse {
    data: Value,
    status: Status,
}

impl APIResponse {
    /// Set the data of the `Response` to `data`.
    pub fn data(mut self, data: Value) -> APIResponse {
        self.data = data;
        self
    }

    /// Convenience method to set `self.data` to `{"message": message}`.
    pub fn message(mut self, message: &str) -> APIResponse {
        self.data = json!({ "message": message });
        self
    }
}

impl From<DieselError> for APIResponse {
    fn from(_: DieselError) -> Self {
        APIResponse::from(InternalServerError { data: "Internal server error" })
    }
}

impl<'r> Responder<'r> for APIResponse {
    fn respond_to(self, _req: &Request) -> Result<Response<'r>, Status> {
        let body = self.data;

        Response::build()
            .status(self.status)
            .sized_body(body.to_string().len(), Cursor::new(body.to_string()))
            .header(ContentType::JSON)
            .ok()
    }
}


pub fn ok() -> APIResponse {
    APIResponse {
        data: json!(null),
        status: Status::Ok,
    }
}

pub fn created() -> APIResponse {
    APIResponse {
        data: json!(null),
        status: Status::Created,
    }
}

pub fn accepted() -> APIResponse {
    APIResponse {
        data: json!(null),
        status: Status::Accepted,
    }
}

pub fn no_content() -> APIResponse {
    APIResponse {
        data: json!(null),
        status: Status::NoContent,
    }
}

#[derive(Debug)]
pub struct BadRequest<'a> {
    pub data: &'a str
}

impl From<BadRequest<'_>> for APIResponse {
    fn from(error: BadRequest) -> Self {
        APIResponse {
            data: json!({"message" : error.data}),
            status: Status::BadRequest,
        }
    }
}

#[derive(Debug)]
pub struct Unauthorized<'a> {
    pub data: &'a str
}

impl From<Unauthorized<'_>> for APIResponse {
    fn from(error: Unauthorized) -> Self {
        APIResponse {
            data: json!({"message": error.data}),
            status: Status::Unauthorized,
        }
    }
}

#[derive(Debug)]
pub struct Forbidden<'a> {
    pub data: &'a str
}

impl From<Forbidden<'_>> for APIResponse {
    fn from(error: Forbidden) -> Self {
        APIResponse {
            data: json!({"message": error.data}),
            status: Status::Forbidden,
        }
    }
}

#[derive(Debug)]
pub struct NotFound<'a> {
    pub data: &'a str
}

impl From<NotFound<'_>> for APIResponse {
    fn from(error: NotFound) -> Self {
        APIResponse {
            data: json!({"message": error.data}),
            status: Status::NotFound,
        }
    }
}

#[derive(Debug)]
pub struct NotAllowed<'a> {
    pub data: &'a str
}

impl From<NotAllowed<'_>> for APIResponse {
    fn from(_: NotAllowed) -> Self {
        unimplemented!()
    }
}

#[derive(Debug)]
pub struct Conflict<'a> {
    pub data: &'a str
}

impl From<Conflict<'_>> for APIResponse {
    fn from(_: Conflict) -> Self {
        unimplemented!()
    }
}

#[derive(Debug)]
pub struct UnprocessableEntity<'a> {
    pub data: &'a str
}

impl From<UnprocessableEntity<'_>> for APIResponse {
    fn from(_: UnprocessableEntity) -> Self {
        unimplemented!()
    }
}

#[derive(Debug)]
pub struct InternalServerError<'a> {
    pub data: &'a str
}

impl From<InternalServerError<'_>> for APIResponse {
    fn from(_: InternalServerError) -> Self {
        unimplemented!()
    }
}

#[derive(Debug)]
pub struct ServiceUnavailable<'a> {
    pub data: &'a str
}

impl From<ServiceUnavailable<'_>> for APIResponse {
    fn from(_: ServiceUnavailable) -> Self {
        unimplemented!()
    }
}