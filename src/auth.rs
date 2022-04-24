use std::future::Future;
use std::pin::Pin;

use crate::errors::AppError;
use crate::models::users::User;
use crate::{ACCESS_TOKEN_SECRET, REFRESH_TOKEN_SECRET};

use actix_web::cookie::Cookie;
use actix_web::FromRequest;
use jwt_simple::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
struct AccessTokenClaims {
    user_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RefreshTokenClaims {
    pub user_id: i32,
    pub token_version: i32,
}

pub fn generate_access_token(user: &User) -> Result<String, AppError> {
    let key = ACCESS_TOKEN_SECRET.clone();
    let claims_data = AccessTokenClaims { user_id: user.id };
    let claims = Claims::with_custom_claims(claims_data, Duration::from_secs(60));
    let token = key.authenticate(claims)?;
    Ok(token)
}

pub fn generate_refresh_token(user: &User) -> Result<String, AppError> {
    let key = REFRESH_TOKEN_SECRET.clone();
    let claim_data = RefreshTokenClaims {
        user_id: user.id,
        token_version: user.token_version,
    };
    let claims = Claims::with_custom_claims(claim_data, Duration::from_days(7));
    let token = key.authenticate(claims)?;
    Ok(token)
}

pub fn create_refresh_token_cookie(token: &str) -> Cookie {
    Cookie::build("jid", token).http_only(true).finish()
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ValidatedUser {
    pub user_id: i32,
}

impl FromRequest for ValidatedUser {
    type Error = AppError;
    type Future = Pin<Box<dyn Future<Output = Result<ValidatedUser, AppError>>>>;
    type Config = ();

    fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let auth = req.headers().get("Authorization");
        match auth {
            Some(s) => {
                let split: Vec<&str> = s.to_str().unwrap().split_whitespace().collect();
                let token = if split.len() == 2 {
                    split[1].trim()
                } else {
                    return Box::pin(async {
                        Err(AppError::Unauthorized(String::from(
                            "Malformed authorization header",
                        )))
                    });
                };
                let key = ACCESS_TOKEN_SECRET.clone();
                let data = key.verify_token::<ValidatedUser>(token, None);
                match data {
                    Ok(s) => Box::pin(async { Ok(s.custom) }),
                    Err(_) => Box::pin(async {
                        Err(AppError::Unauthorized(String::from("Bad access token")))
                    }),
                }
            }
            None => Box::pin(async {
                Err(AppError::Unauthorized(String::from(
                    "No authorization header",
                )))
            }),
        }
    }
}
