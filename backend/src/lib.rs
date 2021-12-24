#![feature(proc_macro_hygiene, decl_macro)]
#![recursion_limit = "128"]

#[macro_use]
extern crate rocket;

use rocket::{Build, catchers, Ignite, routes};

mod schema;
mod config;
mod responses;
mod handlers;

/// Constructs a new Rocket instance.
///
/// This function takes care of attaching all routes and handlers of the application.
pub fn rocket_factory(config_name: &str) -> Result<rocket::Rocket<Build>, String> {
    let (app_config, rocket_config) =
        config::get_rocket_config(config_name).map_err(|x| format!("{}", x))?;
    let rocket = rocket::custom(rocket_config)
        .attach(database::DbConn::fairing())
        .manage(app_config)
        .mount("/hello/", routes![api::hello::whoami])
        .mount("/auth/", routes![api::auth::login, api::auth::register,])
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