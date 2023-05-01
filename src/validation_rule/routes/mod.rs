use rocket::{post, serde::json::Json};
use serde_json::Value;

use crate::database_setup::ConnectionPg;
use crate::error_responder::ApiErrorResponse;
use crate::validation_rule::handlers::ValidationRuleHandler;
use crate::validation_rule::models::validation_rule::ValidationRuleDto;

#[post("/rule/<rule_id>")]
pub async fn get_rule(
    rule_id: String,
    mut db_pool: ConnectionPg,
) -> Result<Value, ApiErrorResponse> {
    ValidationRuleHandler::from(&mut *db_pool)
        .get_rule_handler(rule_id)
        .await
}

#[post("/rule/create", format = "json", data = "<new_rule>")]
pub async fn create_rule(
    new_rule: Json<ValidationRuleDto>,
    mut db_pool: ConnectionPg,
) -> Result<Value, ApiErrorResponse> {
    ValidationRuleHandler::from(&mut *db_pool)
        .create_rule_handler(new_rule.0)
        .await
}
