use crate::schema::advance_options;
use crate::validation_rule::models::validation_rule::ValidationRule;
use diesel::{Associations, Identifiable, Insertable, Queryable};
use serde::Serialize;
use serde_derive::Deserialize;
use uuid::Uuid;

#[derive(Queryable, Associations, Identifiable, Debug)]
#[diesel(belongs_to(ValidationRule))]
#[diesel(table_name = advance_options)]
pub struct AdvanceOption {
    pub id: Uuid,
    pub validation_rule_id: Uuid,
    pub is_email_validate: bool,
}

#[derive(Insertable)]
#[diesel(table_name = advance_options)]
pub struct NewAdvanceOption<'a> {
    pub validation_rule_id: &'a Uuid,
    pub is_email_validate: bool,
}

#[derive(Serialize, Deserialize)]
pub struct AdvanceOptionDto {
    pub extension: i32,
    pub keywords: Vec<String>,
    pub is_email_validate: bool,
}
