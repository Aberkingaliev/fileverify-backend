use crate::schema::keywords_for_options;
use crate::validation_rule::models::advance_option::AdvanceOption;
use diesel::prelude::*;
use diesel::{Associations, Identifiable, Queryable};
use uuid::Uuid;

#[derive(Queryable, Associations, Identifiable)]
#[diesel(belongs_to(AdvanceOption))]
#[diesel(table_name = keywords_for_options)]
pub struct KeywordForOption {
    pub id: Uuid,
    pub advance_option_id: Uuid,
    pub keyword: String,
}

#[derive(Insertable)]
#[diesel(table_name = keywords_for_options)]
pub struct NewKeywordForOption<'a> {
    pub advance_option_id: &'a Uuid,
    pub keyword: String,
}
