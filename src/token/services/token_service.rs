use crate::{
    schema::tokens,
    token::models::{Token, TokenDto},
};
use diesel::prelude::*;
use diesel::result::Error;
use diesel::PgConnection;
use uuid::Uuid;

pub struct TokenService<'a> {
    connection: &'a mut PgConnection,
}

impl<'a> TokenService<'a> {
    pub fn new(connection: &'a mut PgConnection) -> Self {
        return TokenService { connection };
    }
}

impl<'a> TokenService<'a> {
    pub async fn create_token(self, new_token: &TokenDto<'_>) -> usize {
        let token_inserted = diesel::insert_into(tokens::table)
            .values(new_token)
            .execute(self.connection)
            .expect("Error duration insert token");

        return token_inserted;
    }

    pub async fn update_token(self, user_id: &Uuid, refresh_token: &String) -> usize {
        let updated_token = diesel::update(tokens::table.filter(tokens::user_id.eq(user_id)))
            .set(tokens::refresh_token.eq(refresh_token))
            .execute(self.connection)
            .expect("Error duration update token");

        return updated_token;
    }

    pub async fn delete_token(self, ref_token: String) -> usize {
        let deleted_token =
            diesel::delete(tokens::table.filter(tokens::refresh_token.eq(ref_token)))
                .execute(self.connection)
                .expect("Error during deleting token");
        return deleted_token;
    }

    pub async fn get_token_by_refresh(
        self,
        ref_token: String,
    ) -> Result<Token, Box<dyn std::error::Error>> {
        let founded_token = tokens::table
            .filter(tokens::refresh_token.eq(ref_token))
            .get_result::<Token>(self.connection)
            .expect("Error during request DB");

        return Ok(founded_token);
    }

    pub async fn get_token_by_user_id(self, user_id: &Uuid) -> Result<Token, Error> {
        let founded_token = tokens::table
            .filter(tokens::user_id.eq(user_id))
            .get_result::<Token>(self.connection);

        return founded_token;
    }
}
