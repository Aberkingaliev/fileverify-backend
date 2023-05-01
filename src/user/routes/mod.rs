use rocket::{get, response::Redirect};

use crate::{database_setup::ConnectionPg, error_responder::ApiErrorResponse};

use super::handlers::UserHandler;

#[get("/activate/<link>")]
pub async fn activate(
    link: String,
    mut db_pool: ConnectionPg,
) -> Result<Redirect, ApiErrorResponse> {
    UserHandler::from(&mut *db_pool)
        .activate_account_handler(link)
        .await
}
