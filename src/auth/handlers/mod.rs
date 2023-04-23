use diesel::PgConnection;
use rocket::http::{Cookie, CookieJar};
use serde_json::{json, Value};

use crate::{
    error_responder::ApiErrorResponse,
    token::services::TokenService,
    user::models::{UserLoginDto, UserRegistrationDto},
};

use super::{api_errors::unauthorized, services::AuthService, utils::AuthUtils};

pub struct AuthHandler<'a> {
    connection: &'a mut PgConnection,
    cookies: &'a CookieJar<'a>,
}

impl<'a> AuthHandler<'a> {
    pub fn new(connection: &'a mut PgConnection, cookies: &'a CookieJar<'a>) -> Self {
        AuthHandler {
            connection,
            cookies,
        }
    }
}

impl<'a> AuthHandler<'a> {
    pub async fn user_registration_handler(
        self,
        user: UserRegistrationDto,
    ) -> Result<Value, ApiErrorResponse> {
        let registration_response = match AuthService::from(self.connection)
            .user_registration(user)
            .await
        {
            Ok(user) => user,
            Err(api_response) => return Err(api_response),
        };
        let json = json!(&registration_response);
        let builded_cookies = AuthUtils::set_token_cookie(
            "refresh_token".to_string(),
            registration_response.tokens.refresh_token,
        );
        self.cookies.add(builded_cookies);
        return Ok(json);
    }

    pub async fn user_login_handler(self, user: UserLoginDto) -> Result<Value, ApiErrorResponse> {
        let login_response = match AuthService::from(self.connection).user_login(user).await {
            Ok(user) => user,
            Err(api_response) => return Err(api_response),
        };

        let json = json!(&login_response);
        let builded_cookie = AuthUtils::set_token_cookie(
            "refresh_token".to_string(),
            login_response.tokens.refresh_token,
        );
        self.cookies.add(builded_cookie);

        return Ok(json);
    }

    pub async fn user_logout_handler(self) {
        let token = self
            .cookies
            .get("refresh_token")
            .unwrap()
            .value()
            .to_string();
        TokenService::new(self.connection).delete_token(token).await;
        self.cookies.remove(Cookie::named("refresh_token"));
    }

    pub async fn user_refresh_handler(self) -> Result<Value, ApiErrorResponse> {
        let refresh_token = match self.cookies.get("refresh_token") {
            Some(cookie_token) => cookie_token.value().to_string(),
            None => return Err(unauthorized()),
        };
        let refresh_response = match AuthService::from(self.connection)
            .refresh_user(refresh_token)
            .await
        {
            Ok(user) => user,
            Err(api_response) => return Err(api_response),
        };
        let json = serde_json::json!(&refresh_response);
        let builded_cookies = AuthUtils::set_token_cookie(
            "refresh_token".to_string(),
            refresh_response.tokens.refresh_token,
        );
        self.cookies.add(builded_cookies);
        return Ok(json);
    }
}
