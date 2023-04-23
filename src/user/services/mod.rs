use crate::{
    schema::users,
    schema::users::dsl::*,
    user::models::{User, UserRegistrationDto},
    ApiErrorResponse,
};

use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::Error;
use diesel::RunQueryDsl;
use rocket::response::Redirect;
use uuid::Uuid;

use super::api_errors::invalid_activation_link;

pub struct UserService<'a> {
    connection: &'a mut PgConnection,
}

impl<'a> UserService<'a> {
    pub fn new(connection: &'a mut PgConnection) -> Self {
        UserService { connection }
    }
}

impl<'a> From<&'a mut PgConnection> for UserService<'a> {
    fn from(connection: &'a mut PgConnection) -> Self {
        UserService { connection }
    }
}

impl<'a> UserService<'a> {
    pub async fn activate_account(
        &'a mut self,
        link: String,
    ) -> Result<Redirect, ApiErrorResponse> {
        match users
            .filter(activation_link.eq(&link))
            .get_result::<User>(self.connection)
        {
            Ok(_) => match self.update_activation_link(link).await {
                Ok(_) => return Ok(Redirect::to("https://google.com")),
                Err(_) => return Err(invalid_activation_link()),
            },
            Err(_) => return Err(invalid_activation_link()),
        }
    }

    pub async fn update_activation_link(&'a mut self, link: String) -> Result<usize, Error> {
        let update_procedure = diesel::update(users::table.filter(users::activation_link.eq(link)))
            .set(users::is_activated.eq(true))
            .execute(self.connection);
        return update_procedure;
    }

    pub async fn create_user(&'a mut self, user: &UserRegistrationDto) -> Result<User, Error> {
        let user_inserted = diesel::insert_into(users::table)
            .values(user)
            .get_result::<User>(self.connection);

        return user_inserted;
    }

    pub async fn get_user_by_email(&'a mut self, u_email: &String) -> Result<User, Error> {
        let user_founded = users
            .filter(email.eq(u_email))
            .get_result::<User>(self.connection);

        return user_founded;
    }

    pub async fn get_user_by_id(&'a mut self, user_id: &Uuid) -> Result<User, Error> {
        let user_founded = users.find(user_id).get_result::<User>(self.connection);

        return user_founded;
    }

    pub async fn check_user_by_email(&'a mut self, u_email: &String) -> bool {
        let user_founded = users
            .filter(email.eq(u_email))
            .get_result::<User>(self.connection);

        let result = match user_founded {
            Ok(_) => true,
            Err(_) => false,
        };
        return result;
    }
}
