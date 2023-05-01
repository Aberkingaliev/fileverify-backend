use diesel::PgConnection;
use serde_json::{json, Value};

use crate::error_responder::ApiErrorResponse;
use crate::validation_rule::models::validation_rule::ValidationRuleDto;
use crate::validation_rule::services::{
    ValidationRuleCreateResult, ValidationRuleGetResult, ValidationRuleService,
};

pub struct ValidationRuleHandler<'a> {
    connection: &'a mut PgConnection,
}

impl<'a> From<&'a mut PgConnection> for ValidationRuleHandler<'a> {
    fn from(connection: &'a mut PgConnection) -> Self {
        ValidationRuleHandler { connection }
    }
}

impl<'a> ValidationRuleHandler<'a> {
    pub async fn get_rule_handler(self, rule_id: String) -> Result<Value, ApiErrorResponse> {
        match ValidationRuleService::from(self.connection)
            .get_rule_by_id(rule_id)
            .await
        {
            ValidationRuleGetResult::Some(validation_rule) => return Ok(json!(validation_rule)),
            ValidationRuleGetResult::NotFound(message) => {
                return Err(ApiErrorResponse::not_found(message))
            }
            ValidationRuleGetResult::UnexpectedError(message) => {
                return Err(ApiErrorResponse::internal_server_error(message))
            }
            ValidationRuleGetResult::InvalidUuid(message) => {
                return Err(ApiErrorResponse::bad_request(message))
            }
        };
    }

    pub async fn create_rule_handler(
        self,
        new_rule: ValidationRuleDto,
    ) -> Result<Value, ApiErrorResponse> {
        match ValidationRuleService::from(self.connection)
            .create_validation_rule(new_rule)
            .await
        {
            ValidationRuleCreateResult::Ok(rule) => return Ok(json!(rule)),
            ValidationRuleCreateResult::UnexpectedError(message) => {
                return Err(ApiErrorResponse::internal_server_error(message))
            }
        }
    }
}
