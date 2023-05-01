use diesel::PgConnection;
use rocket::response::Redirect;

use crate::error_responder::ApiErrorResponse;
use crate::user::services::{UserActivationResult, UserService};

pub struct UserHandler<'a> {
    connection: &'a mut PgConnection,
}

impl<'a> From<&'a mut PgConnection> for UserHandler<'a> {
    fn from(connection: &'a mut PgConnection) -> Self {
        UserHandler { connection }
    }
}

impl<'a> UserHandler<'a> {
    pub async fn activate_account_handler(
        self,
        link: String,
    ) -> Result<Redirect, ApiErrorResponse> {
        match UserService::from(self.connection)
            .activate_account(link)
            .await
        {
            UserActivationResult::Ok(redirect) => return Ok(redirect),
            UserActivationResult::InvalidActivationLink(message) => {
                return Err(ApiErrorResponse::bad_request(message))
            }
            UserActivationResult::UnexpectedError(message) => {
                return Err(ApiErrorResponse::internal_server_error(message))
            }
        }
    }
}
