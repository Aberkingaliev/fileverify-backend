use crate::schema::users;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, PartialEq, Identifiable, Deserialize, Serialize, Debug)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub username: String,
    pub email: String,
    pub password: String,
    pub is_activated: bool,
    pub activation_link: String,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = users)]
pub struct UserRegistrationDto {
    pub name: String,
    pub username: String,
    pub email: String,
    pub password: String,
    pub activation_link: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserPayloadDto {
    pub name: String,
    pub username: String,
    pub email: String,
    pub is_activated: bool,
}

#[derive(Serialize, Deserialize)]
pub struct UserLoginDto {
    pub email: String,
    pub password: String,
}

impl From<User> for UserPayloadDto {
    fn from(value: User) -> Self {
        UserPayloadDto {
            name: value.name,
            username: value.username,
            email: value.email,
            is_activated: value.is_activated,
        }
    }
}

impl From<UserRegistrationDto> for UserPayloadDto {
    fn from(value: UserRegistrationDto) -> Self {
        UserPayloadDto {
            name: value.name,
            username: value.username,
            email: value.email,
            is_activated: false,
        }
    }
}
