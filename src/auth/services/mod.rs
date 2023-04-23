use crate::{
    error_responder::ApiErrorResponse,
    mail::services::MailService,
    token::models::TokenDto,
    token::services::{JwtService, TokenBundle, TokenService},
    user::models::{UserLoginDto, UserPayloadDto, UserRegistrationDto},
    user::services::UserService,
};
use bcrypt::{hash, verify};
use diesel::PgConnection;
use serde::{Deserialize, Serialize};

use super::api_errors::{
    failed_create_user, invalid_password, user_already_registred, user_not_found,
};

#[derive(Serialize, Deserialize)]
pub struct AuthResponse {
    pub tokens: TokenBundle,
    user: UserPayloadDto,
}

pub struct AuthService<'a> {
    connection: &'a mut PgConnection,
}

impl<'a> From<&'a mut PgConnection> for AuthService<'a> {
    fn from(connection: &'a mut PgConnection) -> Self {
        AuthService { connection }
    }
}

impl<'a> AuthService<'a> {
    pub async fn user_registration(
        &'a mut self,
        user: UserRegistrationDto,
    ) -> Result<AuthResponse, ApiErrorResponse> {
        let current_user = UserService::new(self.connection)
            .check_user_by_email(&user.email)
            .await;

        if current_user {
            return Err(user_already_registred());
        }

        let hashed_password = hash(&user.password, 10).expect("Error: Hashing Password");
        let activation_link = uuid::Uuid::new_v4().to_string();
        let registration_dto = UserRegistrationDto {
            password: hashed_password,
            activation_link: Some(activation_link),
            ..user
        };
        let created_user = match UserService::new(self.connection)
            .create_user(&registration_dto)
            .await
        {
            Ok(created_user) => {
                let activation_link = &created_user.activation_link;
                let username = &created_user.username;
                let html_body = format!(
                "<html><body><p>Dear {},</p>\
                <p>Thank you for creating an account. Please click on the following link to activate your account:</p>\
                <p><a href=\"http://localhost:8000/api-v1/activate/{}\">http://localhost:8000/api-v1/activate/{}</a></p>\
                <p>Best regards,</p>\
                <p>The FileVerify Team</p></body></html>",
                username, activation_link, activation_link
                );
                let mut message = MailService {
                    receiver: &created_user.email.as_str(),
                    subject: "Account activation",
                    html_body: html_body.as_str(),
                };
                message.send().await;
                created_user
            }
            Err(_) => return Err(failed_create_user()),
        };
        let user_payload = UserPayloadDto {
            ..registration_dto.into()
        };
        let generated_tokens = JwtService::generate_tokens(&user_payload)
            .await
            .expect("Error during token generation");
        let new_token = TokenDto {
            user_id: &created_user.id,
            refresh_token: &generated_tokens.refresh_token,
        };

        TokenService::new(self.connection)
            .create_token(&new_token)
            .await;

        Ok(AuthResponse {
            tokens: generated_tokens,
            user: user_payload,
        })
    }

    pub async fn user_login(
        &'a mut self,
        user_credintals: UserLoginDto,
    ) -> Result<AuthResponse, ApiErrorResponse> {
        let current_user = match UserService::new(self.connection)
            .get_user_by_email(&user_credintals.email)
            .await
        {
            Ok(user) => user,
            Err(_) => return Err(user_not_found()),
        };

        let is_password_valid = verify(user_credintals.password, &current_user.password)
            .expect("Error verifying password");

        if !is_password_valid {
            return Err(invalid_password());
        }

        let payload = UserPayloadDto {
            name: current_user.name,
            username: current_user.username,
            email: current_user.email,
            is_activated: current_user.is_activated,
        };

        let generated_tokens = JwtService::generate_tokens(&payload)
            .await
            .expect("Token generation error");

        if let Ok(token) = TokenService::new(self.connection)
            .get_token_by_user_id(&current_user.id)
            .await
        {
            TokenService::new(self.connection)
                .update_token(&token.user_id, &generated_tokens.refresh_token)
                .await;
        } else {
            let new_token = TokenDto {
                user_id: &current_user.id,
                refresh_token: &generated_tokens.refresh_token,
            };

            TokenService::new(self.connection)
                .create_token(&new_token)
                .await;
        }

        Ok(AuthResponse {
            tokens: generated_tokens,
            user: payload,
        })
    }

    pub async fn refresh_user(
        &'a mut self,
        refresh_token: String,
    ) -> Result<AuthResponse, ApiErrorResponse> {
        let current_token = TokenService::new(self.connection)
            .get_token_by_refresh(refresh_token)
            .await
            .expect("Error during find token");

        let validation_token = JwtService::validate_token(&current_token.refresh_token)
            .expect("Error validating token");
        let generated_tokens = JwtService::generate_tokens(&validation_token)
            .await
            .expect("Error generating token");

        if let Ok(user) = UserService::new(self.connection)
            .get_user_by_id(&current_token.user_id)
            .await
        {
            TokenService::new(self.connection)
                .update_token(&user.id, &generated_tokens.refresh_token)
                .await
        } else {
            let new_token = &TokenDto {
                user_id: &current_token.user_id,
                refresh_token: &generated_tokens.refresh_token,
            };
            TokenService::new(self.connection)
                .create_token(&new_token)
                .await
        };

        Ok(AuthResponse {
            tokens: generated_tokens,
            user: validation_token,
        })
    }
}
