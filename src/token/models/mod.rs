use crate::schema::tokens;
use diesel::{Insertable, Queryable};
use uuid::Uuid;

#[derive(Queryable, Debug)]
pub struct Token {
    pub id: Uuid,
    pub user_id: Uuid,
    pub refresh_token: String,
}

#[derive(Insertable)]
#[diesel(table_name = tokens)]
pub struct TokenDto<'a> {
    pub user_id: &'a Uuid,
    pub refresh_token: &'a String,
}
