use crate::schema::extension_for_rules;
use crate::validation_rule::models::advance_option::AdvanceOption;
use crate::validation_rule::models::validation_rule::ValidationRule;
use diesel::prelude::*;
use diesel::{Associations, Identifiable, Queryable};
use uuid::Uuid;

#[derive(Queryable, Associations, Identifiable, Debug)]
#[diesel(belongs_to(ValidationRule))]
#[diesel(belongs_to(AdvanceOption))]
#[diesel(table_name = extension_for_rules)]
pub struct ExtensionForRules {
    pub id: Uuid,
    pub validation_rule_id: Uuid,
    pub advance_option_id: Option<Uuid>,
    pub extension_id: i32,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = extension_for_rules)]
pub struct NewExtensionForRules {
    pub validation_rule_id: Uuid,
    pub advance_option_id: Option<Uuid>,
    pub extension_id: i32,
}
