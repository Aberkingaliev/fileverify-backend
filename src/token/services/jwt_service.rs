use chrono::{Duration, Utc};
use dotenv::dotenv;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::user::models::UserPayloadDto;
use std::{borrow::Cow, env};

use crate::token::services::TokenBundle;

pub enum TimeForm {
    Days,
    Minutes,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserClaims<'a> {
    pub user: Cow<'a, UserPayloadDto>,
    pub exp: i64,
    pub iat: i64,
}

impl<'a> UserClaims<'a> {
    fn new(user: Cow<'a, UserPayloadDto>, expir_time: i64, time_form: TimeForm) -> Self {
        match time_form {
            TimeForm::Days => {
                return UserClaims {
                    user,
                    exp: (Utc::now() + Duration::days(expir_time)).timestamp(),
                    iat: Utc::now().timestamp(),
                }
            }
            TimeForm::Minutes => {
                return UserClaims {
                    user,
                    exp: (Utc::now() + Duration::minutes(expir_time)).timestamp(),
                    iat: Utc::now().timestamp(),
                }
            }
        }
    }
}

pub struct JwtService {}

impl JwtService {
    pub async fn generate_tokens(
        user: &UserPayloadDto,
    ) -> Result<TokenBundle, Box<dyn std::error::Error>> {
        dotenv().ok();
        let dto_for_access = UserClaims::new(Cow::Borrowed(user), 30, TimeForm::Minutes);
        let dto_for_refresh = UserClaims::new(Cow::Borrowed(user), 30, TimeForm::Days);
        let header = Header::new(Algorithm::HS256);
        let key = env::var("SECRET_KEY").expect("There is no key to signing");
        let access_token = encode(
            &header,
            &dto_for_access,
            &EncodingKey::from_secret(key.as_ref()),
        )
        .expect("Error during token generation");
        let refresh_token = encode(
            &header,
            &dto_for_refresh,
            &EncodingKey::from_secret(key.as_ref()),
        )
        .expect("Error during token generation");

        let token_bundle = TokenBundle {
            access_token,
            refresh_token,
        };

        return Ok(token_bundle);
    }

    pub fn validate_token(token: &String) -> Result<UserPayloadDto, Box<dyn std::error::Error>> {
        let key = env::var("SECRET_KEY").expect("There is no key to signing");

        let validation = &Validation::new(Algorithm::HS256);
        let decoded_data =
            decode::<UserClaims>(token, &DecodingKey::from_secret(key.as_ref()), &validation)
                .map_err(|err| {
                    println!("Error while decoding token: {:?}", token);
                    err
                })?;

        let claim = decoded_data.claims.user;
        Ok(claim.into_owned())
    }
}
