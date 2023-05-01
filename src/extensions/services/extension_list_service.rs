use crate::extensions::models::extension_list::{Extension, NewExtension};
use crate::schema::extension_list;
use diesel::prelude::*;
use diesel::QueryDsl;
use diesel::{result::Error, PgConnection, RunQueryDsl};

pub struct ExtensionListService<'a> {
    pub connection: &'a mut PgConnection,
}

impl<'a> From<&'a mut PgConnection> for ExtensionListService<'a> {
    fn from(connection: &'a mut PgConnection) -> Self {
        return ExtensionListService { connection };
    }
}

impl<'a> ExtensionListService<'a> {
    pub fn create_extension(&'a mut self, extension: &NewExtension) -> Result<usize, Error> {
        let result = diesel::insert_into(extension_list::table)
            .values(extension)
            .execute(self.connection);
        return result;
    }

    pub fn get_extension_by_id(&'a mut self, id: &i32) -> Result<Extension, Error> {
        let result = extension_list::table
            .find(id)
            .get_result::<Extension>(self.connection);
        return result;
    }

    pub fn get_extension_by_value(&'a mut self, value: &String) -> Result<Extension, Error> {
        let result = extension_list::table
            .filter(extension_list::extension.eq(value))
            .get_result::<Extension>(self.connection);
        return result;
    }
}
