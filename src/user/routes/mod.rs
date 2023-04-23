use rocket::{get, response::Redirect};

use crate::{database_setup::ConnectionPg, error_responder::ApiErrorResponse};

use super::services::UserService;

#[get("/activate/<link>")]
pub async fn activate(
    link: String,
    mut db_pool: ConnectionPg,
) -> Result<Redirect, ApiErrorResponse> {
    UserService::from(&mut *db_pool)
        .activate_account(link)
        .await
}
