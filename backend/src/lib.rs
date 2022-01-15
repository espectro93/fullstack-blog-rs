#![feature(proc_macro_hygiene, decl_macro)]
#![recursion_limit = "128"]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;

use rocket::{Build, catchers, Ignite, routes};

mod schema;
mod responses;
mod handlers;
mod models;
mod database;
mod services;
mod api;
mod validation;

/// Constructs a new Rocket instance.
///
/// This function takes care of attaching all routes and handlers of the application.
pub fn rocket_factory() -> Result<rocket::Rocket<Build>, String> {
    let rocket = rocket::build()
        .attach(database::DbConn::fairing())
        .manage(app_config)
        .mount("/auth/", routes![api::auth::login, api::auth::register])
        .register(catchers![
            handlers::bad_request_handler,
            handlers::unauthorized_handler,
            handlers::forbidden_handler,
            handlers::not_found_handler,
            handlers::internal_server_error_handler,
            handlers::service_unavailable_handler,
        ], ());
    Ok(rocket)
}