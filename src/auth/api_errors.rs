use rocket::http::Status;

use crate::error_responder::ApiErrorResponse;

pub fn unauthorized() -> ApiErrorResponse {
    ApiErrorResponse {
        detail: "User is not authorized".to_string(),
        status: Status::Unauthorized,
    }
}

pub fn failed_create_user() -> ApiErrorResponse {
    ApiErrorResponse {
        detail: "Failed to create user".to_string(),
        status: Status::BadRequest,
    }
}

pub fn user_not_found() -> ApiErrorResponse {
    ApiErrorResponse {
        detail: "User is not found".to_string(),
        status: Status::BadRequest,
    }
}

pub fn user_already_registred() -> ApiErrorResponse {
    ApiErrorResponse {
        detail: "User is already registered".to_string(),
        status: Status::BadRequest,
    }
}

pub fn invalid_password() -> ApiErrorResponse {
    ApiErrorResponse {
        detail: "User entered the wrong password".to_string(),
        status: Status::BadRequest,
    }
}
