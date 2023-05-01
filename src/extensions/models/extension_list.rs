use crate::schema::extension_list;
use diesel::prelude::*;
use diesel::Queryable;

#[derive(Queryable)]
#[diesel(table_name = extension_list)]
pub struct Extension {
    pub id: i32,
    pub extension: String,
}

#[derive(Insertable)]
#[diesel(table_name = extension_list)]
pub struct NewExtension {
    pub extension: String,
}
