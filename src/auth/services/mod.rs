use crate::{
    auth::api_errors::{
        INVALID_PASSWORD, PASSWORD_HASHING_ERROR, PASSWORD_VERIFICATION_ERROR,
        TOKEN_GENERATION_ERROR, TOKEN_VALIDATION_ERROR, UNAUTHORIZED, UNEXPECT_DB_ERROR,
        USER_ALREADT_EXIST, USER_NOT_FOUND,
    },
    mail::message_layouts::activation_message,
    mail::services::MailService,
    token::models::TokenDto,
    token::services::{JwtService, TokenBundle, TokenService},
    user::models::{UserLoginDto, UserPayloadDto, UserRegistrationDto},
    user::services::UserService,
};
use bcrypt::{hash, verify};
use diesel::{result::Error, PgConnection};
use jsonwebtoken::errors::ErrorKind;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AuthResponse {
    pub tokens: TokenBundle,
    user: UserPayloadDto,
}

pub struct AuthService<'a> {
    connection: &'a mut PgConnection,
}

pub enum AuthLoginResult {
    Ok(AuthResponse),
    NotFound(&'static str),
    InvalidPassword(&'static str),
    UnexpectedError(&'static str),
}

pub enum AuthRegistrationResult {
    Ok(AuthResponse),
    AlreadyRegistred(&'static str),
    UnexpectedError(&'static str),
}

pub enum AuthRefreshResult {
    Ok(AuthResponse),
    Unauthorized(&'static str),
    UnexpectedError(&'static str),
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
    ) -> AuthRegistrationResult {
        let current_user = UserService::new(self.connection)
            .check_user_by_email(&user.email)
            .await;

        if current_user {
            return AuthRegistrationResult::AlreadyRegistred(USER_ALREADT_EXIST);
        }

        let hashed_password = match hash(&user.password, 10) {
            Ok(password) => password,
            Err(_) => return AuthRegistrationResult::UnexpectedError(PASSWORD_HASHING_ERROR),
        };
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
                let html_body =
                    activation_message(&created_user.name, &created_user.activation_link);
                let mut message = MailService {
                    receiver: &created_user.email.as_str(),
                    subject: "Account activation",
                    html_body: html_body.as_str(),
                };
                message.send().await;
                created_user
            }
            Err(_) => return AuthRegistrationResult::UnexpectedError(UNEXPECT_DB_ERROR),
        };
        let user_payload = UserPayloadDto {
            ..registration_dto.into()
        };
        let generated_tokens = match JwtService::generate_tokens(&user_payload).await {
            Ok(tokens) => tokens,
            Err(_) => return AuthRegistrationResult::UnexpectedError(TOKEN_GENERATION_ERROR),
        };
        let new_token = TokenDto {
            user_id: &created_user.id,
            refresh_token: &generated_tokens.refresh_token,
        };

        TokenService::new(self.connection)
            .create_token(&new_token)
            .await;

        AuthRegistrationResult::Ok(AuthResponse {
            tokens: generated_tokens,
            user: user_payload,
        })
    }

    pub async fn user_login(&'a mut self, user_credintals: UserLoginDto) -> AuthLoginResult {
        let current_user = match UserService::new(self.connection)
            .get_user_by_email(&user_credintals.email)
            .await
        {
            Ok(user) => user,
            Err(_) => return AuthLoginResult::NotFound(USER_NOT_FOUND),
        };

        match verify(user_credintals.password, &current_user.password) {
            Ok(bool) => {
                if !bool {
                    return AuthLoginResult::InvalidPassword(INVALID_PASSWORD);
                }
            }
            Err(_) => return AuthLoginResult::UnexpectedError(PASSWORD_VERIFICATION_ERROR),
        };

        let payload = UserPayloadDto {
            name: current_user.name,
            email: current_user.email,
            is_activated: current_user.is_activated,
        };

        let generated_tokens = match JwtService::generate_tokens(&payload).await {
            Ok(tokens) => tokens,
            Err(_) => return AuthLoginResult::UnexpectedError(TOKEN_GENERATION_ERROR),
        };

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

        AuthLoginResult::Ok(AuthResponse {
            tokens: generated_tokens,
            user: payload,
        })
    }

    pub async fn refresh_user(&'a mut self, refresh_token: String) -> AuthRefreshResult {
        let current_token = match TokenService::new(self.connection)
            .get_token_by_refresh(refresh_token)
            .await
        {
            Ok(token) => token,
            Err(Error::NotFound) => return AuthRefreshResult::Unauthorized(UNAUTHORIZED),
            Err(_) => return AuthRefreshResult::UnexpectedError(UNEXPECT_DB_ERROR),
        };

        let validation_token = match JwtService::validate_token(&current_token.refresh_token) {
            Ok(payload) => payload.claims.user.into_owned(),
            Err(error) => match error.kind() {
                ErrorKind::ExpiredSignature => {
                    return AuthRefreshResult::Unauthorized(UNAUTHORIZED)
                }
                _ => return AuthRefreshResult::UnexpectedError(TOKEN_VALIDATION_ERROR),
            },
        };
        let generated_tokens = match JwtService::generate_tokens(&validation_token).await {
            Ok(tokens) => tokens,
            Err(_) => return AuthRefreshResult::UnexpectedError(TOKEN_GENERATION_ERROR),
        };

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

        AuthRefreshResult::Ok(AuthResponse {
            tokens: generated_tokens,
            user: validation_token,
        })
    }
}
