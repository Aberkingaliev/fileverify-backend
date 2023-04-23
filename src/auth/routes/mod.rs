use crate::{
    database_setup::ConnectionPg,
    error_responder::ApiErrorResponse,
    user::models::{UserLoginDto, UserRegistrationDto},
};

use super::handlers::AuthHandler;
use rocket::http::CookieJar;
use rocket::post;
use rocket::serde::json::Json;
use serde_json::Value;

#[post("/registration", format = "json", data = "<user>")]
pub async fn registration(
    mut db_pool: ConnectionPg,
    user: Json<UserRegistrationDto>,
    cookie_jar: &CookieJar<'_>,
) -> Result<Value, ApiErrorResponse> {
    AuthHandler::new(&mut *db_pool, cookie_jar)
        .user_registration_handler(user.0)
        .await
}

#[post("/login", format = "json", data = "<user_data>")]
pub async fn login(
    mut db_pool: ConnectionPg,
    user_data: Json<UserLoginDto>,
    cookie_jar: &CookieJar<'_>,
) -> Result<Value, ApiErrorResponse> {
    AuthHandler::new(&mut *db_pool, cookie_jar)
        .user_login_handler(user_data.0)
        .await
}

#[post("/logout")]
pub async fn logout(mut db_pool: ConnectionPg, cookie_jar: &CookieJar<'_>) {
    AuthHandler::new(&mut *db_pool, cookie_jar)
        .user_logout_handler()
        .await
}

#[post("/refresh")]
pub async fn refresh_route(
    mut db_pool: ConnectionPg,
    cookie_jar: &CookieJar<'_>,
) -> Result<Value, ApiErrorResponse> {
    AuthHandler::new(&mut *db_pool, cookie_jar)
        .user_refresh_handler()
        .await
}
