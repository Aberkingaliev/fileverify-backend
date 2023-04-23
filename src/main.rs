#![feature(decl_macro)]
#![feature(associated_type_bounds)]
#![feature(impl_trait_projections)]

mod database_setup;
use database_setup::get_connection_pool;
use error_responder::ApiErrorResponse;
use rocket::{catch, catchers, http::Status, launch, routes as rocket_routes};
pub mod schema;
mod user;
use user::routes::activate;
mod auth;
use auth::routes::{login, logout, refresh_route, registration};
mod error_responder;
mod mail;
mod token;
extern crate diesel;
extern crate rocket;

#[catch(422)]
pub fn unprocessable_entity() -> ApiErrorResponse {
    ApiErrorResponse {
        detail: "Incorrect data entered".to_string(),
        status: Status::UnprocessableEntity,
    }
}

#[launch]
async fn rocket() -> _ {
    let pool = get_connection_pool();
    rocket::build()
        .mount(
            "/api-v1/auth",
            rocket_routes![login, registration, logout, refresh_route,],
        )
        .mount("/api-v1/user", rocket_routes![activate])
        .register("/", catchers![unprocessable_entity])
        .manage(pool)
}
