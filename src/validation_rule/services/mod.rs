use crate::schema::{advance_options, extension_for_rules, keywords_for_options, validation_rules};
use crate::validation_rule::api_errors::{INVALID_UUID, RULE_NOT_FOUND, UNEXPECT_DB_ERROR};
use crate::validation_rule::models::advance_option::{
    AdvanceOption, AdvanceOptionDto, NewAdvanceOption,
};
use crate::validation_rule::models::extension_for_rules::{
    ExtensionForRules, NewExtensionForRules,
};
use crate::validation_rule::models::keyword_for_option::{KeywordForOption, NewKeywordForOption};
use crate::validation_rule::models::validation_rule::{ValidationRule, ValidationRuleDto};
use diesel::prelude::*;
use diesel::result::Error;
use uuid::Uuid;

pub struct ValidationRuleService<'a> {
    pub connection: &'a mut PgConnection,
}

pub enum ValidationRuleGetResult {
    Some(ValidationRuleDto),
    NotFound(&'static str),
    InvalidUuid(&'static str),
    UnexpectedError(&'static str),
}

pub enum ValidationRuleCreateResult {
    Ok(ValidationRule),
    UnexpectedError(&'static str),
}

impl<'a> From<&'a mut PgConnection> for ValidationRuleService<'a> {
    fn from(connection: &'a mut PgConnection) -> Self {
        ValidationRuleService { connection }
    }
}

impl<'a> ValidationRuleService<'a> {
    pub async fn get_rule_by_id(&'a mut self, rule_id: String) -> ValidationRuleGetResult {
        let parsed_rule_uuid = match Uuid::parse_str(rule_id.as_str()) {
            Ok(id) => id,
            Err(_) => return ValidationRuleGetResult::InvalidUuid(INVALID_UUID),
        };

        let rule = validation_rules::table
            .find(parsed_rule_uuid)
            .first::<ValidationRule>(self.connection);

        match rule {
            Ok(rule) => {
                let advance_options = match AdvanceOption::belonging_to(&rule)
                    .load::<AdvanceOption>(self.connection)
                {
                    Ok(advance_option) => advance_option,
                    Err(_) => return ValidationRuleGetResult::UnexpectedError(UNEXPECT_DB_ERROR),
                };

                let extensions = match ExtensionForRules::belonging_to(&rule)
                    .select(extension_for_rules::extension_id)
                    .load::<i32>(self.connection)
                {
                    Ok(extensions) => extensions,
                    Err(_) => return ValidationRuleGetResult::UnexpectedError(UNEXPECT_DB_ERROR),
                };

                let advance_options_with_details = advance_options
                    .into_iter()
                    .map(|advance_option| {
                        let keywords = KeywordForOption::belonging_to(&advance_option)
                            .select(keywords_for_options::keyword)
                            .load::<String>(self.connection)
                            .unwrap();
                        let extension_id = ExtensionForRules::belonging_to(&advance_option)
                            .select(extension_for_rules::extension_id)
                            .get_result::<i32>(self.connection)
                            .expect("Database Error");

                        AdvanceOptionDto {
                            extension: extension_id,
                            keywords,
                            is_email_validate: advance_option.is_email_validate,
                        }
                    })
                    .collect::<Vec<AdvanceOptionDto>>();

                ValidationRuleGetResult::Some(ValidationRuleDto {
                    title: rule.title,
                    min_size: rule.min_size,
                    max_size: rule.max_size,
                    allowed_extensions: extensions,
                    advance_option: advance_options_with_details,
                })
            }
            Err(Error::NotFound) => ValidationRuleGetResult::NotFound(RULE_NOT_FOUND),
            Err(_) => ValidationRuleGetResult::UnexpectedError(UNEXPECT_DB_ERROR),
        }
    }

    pub async fn create_validation_rule(
        &'a mut self,
        new_rule: ValidationRuleDto,
    ) -> ValidationRuleCreateResult {
        match self
            .connection
            .transaction::<_, diesel::result::Error, _>(|conn| {
                let validation_rule = match diesel::insert_into(validation_rules::table)
                    .values((
                        validation_rules::title.eq(&new_rule.title),
                        validation_rules::min_size.eq(&new_rule.min_size),
                        validation_rules::max_size.eq(&new_rule.max_size),
                    ))
                    .get_result::<ValidationRule>(conn)
                {
                    Ok(validation_rule) => validation_rule,
                    Err(error) => return Err(error),
                };

                let mut extension_matrix = Vec::new();

                for allowed_extension in new_rule.allowed_extensions {
                    extension_matrix.push((allowed_extension, &validation_rule.id, None::<Uuid>))
                }

                for advance_option in new_rule.advance_option {
                    let new_advance_option = NewAdvanceOption {
                        validation_rule_id: &validation_rule.id,
                        is_email_validate: advance_option.is_email_validate,
                    };

                    let inserted_advance_option = match diesel::insert_into(advance_options::table)
                        .values(new_advance_option)
                        .get_result::<AdvanceOption>(conn)
                    {
                        Ok(advance_option) => advance_option,
                        Err(error) => return Err(error),
                    };

                    for keyword in advance_option.keywords {
                        match diesel::insert_into(keywords_for_options::table)
                            .values(&NewKeywordForOption {
                                advance_option_id: &inserted_advance_option.id,
                                keyword,
                            })
                            .execute(conn)
                        {
                            Ok(keyword) => keyword,
                            Err(error) => return Err(error),
                        };
                    }

                    for mut elem in extension_matrix.iter_mut() {
                        if advance_option.extension == elem.0 {
                            elem.2 = Some(inserted_advance_option.id);
                        }
                    }
                }

                for (extension, validatio_rule_id, advance_option_id) in extension_matrix {
                    let new_extension_for_rules = &NewExtensionForRules {
                        validation_rule_id: *validatio_rule_id,
                        advance_option_id,
                        extension_id: extension,
                    };
                    match diesel::insert_into(extension_for_rules::table)
                        .values(new_extension_for_rules)
                        .execute(conn)
                    {
                        Ok(result) => result,
                        Err(error) => {
                            return Err(error);
                        }
                    };
                }

                return Ok(validation_rule);
            }) {
            Ok(rule) => ValidationRuleCreateResult::Ok(rule),
            Err(_) => {
                return ValidationRuleCreateResult::UnexpectedError(UNEXPECT_DB_ERROR);
            }
        }
    }
}
