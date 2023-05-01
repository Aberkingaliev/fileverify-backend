use crate::schema::validation_rules;
use crate::validation_rule::models::advance_option::AdvanceOptionDto;
use diesel::{Identifiable, Insertable, Queryable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Identifiable, Serialize, Debug)]
#[diesel(table_name = validation_rules)]
pub struct ValidationRule {
    pub id: Uuid,
    pub title: String,
    pub min_size: i64,
    pub max_size: i64,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = validation_rules)]
pub struct NewValidationRule {
    pub title: String,
    pub min_size: i64,
    pub max_size: i64,
}

#[derive(Serialize, Deserialize)]
pub struct ValidationRuleDto {
    pub title: String,
    pub min_size: i64,
    pub max_size: i64,
    pub allowed_extensions: Vec<i32>,
    pub advance_option: Vec<AdvanceOptionDto>,
}
