use std::future::{Future, Ready};
use std::pin::Pin;
use std::process::Output;

use crate::errors::AppError;
use crate::models::User;
use crate::{ACCESS_TOKEN_SECRET, REFRESH_TOKEN_SECRET};
use actix_web::dev::ServiceRequest;
use actix_web::error::ErrorUnauthorized;
use actix_web::{Error, HttpRequest};
use actix_web::{FromRequest, HttpMessage};
use jwt_simple::prelude::*;

pub fn generate_access_token(user: &User) -> Result<String, AppError> {
    let key = ACCESS_TOKEN_SECRET.clone();
    let claims_data = ValidatedUser { user_id: user.id };
    let claims = Claims::with_custom_claims(claims_data, Duration::from_secs(60));
    let token = key.authenticate(claims)?;
    Ok(token)
}

pub fn generate_refresh_token(user: &User) -> Result<String, AppError> {
    let key = REFRESH_TOKEN_SECRET.clone();
    let claim_data = ValidatedUser { user_id: user.id };
    let claims = Claims::with_custom_claims(claim_data, Duration::from_days(7));
    let token = key.authenticate(claims)?;
    Ok(token)
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ValidatedUser {
    pub user_id: i32,
}

impl FromRequest for ValidatedUser {
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<ValidatedUser, Error>>>>;
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
                    return Box::pin(async { Err(ErrorUnauthorized("unauthorized")) });
                };
                let key = ACCESS_TOKEN_SECRET.clone();
                let data = key.verify_token::<ValidatedUser>(token, None);
                match data {
                    Ok(s) => Box::pin(async { Ok(s.custom) }),
                    Err(_) => Box::pin(async { Err(ErrorUnauthorized("unauthorized")) }),
                }
            }
            None => Box::pin(async { Err(ErrorUnauthorized("unauthorized")) }),
        }
    }
}
