use rocket::http::Status;

use crate::ApiErrorResponse;

pub fn invalid_activation_link() -> ApiErrorResponse {
    ApiErrorResponse {
        detail: "Activation link invalid".to_string(),
        status: Status::BadRequest,
    }
}
