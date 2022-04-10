use actix_web::{cookie::Cookie, web, HttpResponse};
use actix_web::{HttpMessage, HttpRequest};
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use diesel::prelude::*;
use jwt_simple::prelude::*;

use crate::auth::{self, ValidatedUser};
use crate::errors::AppError;
use crate::models::UserKey;
use crate::schema::users;
use crate::REFRESH_TOKEN_SECRET;
use crate::{models, Pool};

#[derive(Debug, Deserialize, Serialize)]
struct UserInput {
    username: String,
    password: String,
}

async fn register_user(
    credentials: web::Json<UserInput>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, AppError> {
    let data = credentials.into_inner();
    let username = data.username;
    let password = data.password;
    let user = web::block(move || {
        let conn = &pool.get().unwrap();
        models::create_user(conn, &username, &password)
    })
    .await?;

    let access_token = auth::generate_access_token(&user)?;
    let refresh_token = auth::generate_refresh_token(&user)?;
    let cookie = Cookie::build("jid", refresh_token).http_only(true).finish();
    Ok(HttpResponse::Ok().cookie(cookie).json(TokenResponse {
        ok: true,
        access_token,
    }))
}

#[derive(Debug, Serialize)]
struct TokenResponse {
    ok: bool,
    access_token: String,
}

async fn login_user(
    credentials: web::Json<UserInput>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, AppError> {
    let data = credentials.into_inner();
    let username = data.username;
    let password = data.password;
    let user = web::block(move || {
        let conn = &pool.get().unwrap();
        let user_key = UserKey::Username(username.as_str());
        models::find_user(conn, user_key)
    })
    .await?;

    let hash = PasswordHash::new(&user.password)?;
    let is_correct = Argon2::default()
        .verify_password(password.as_bytes(), &hash)
        .is_ok();
    if !is_correct {
        Err(AppError::AuthError(String::from("Wrong password")))
    } else {
        let access_token = auth::generate_access_token(&user)?;
        let refresh_token = auth::generate_refresh_token(&user)?;
        let cookie = Cookie::build("jid", refresh_token).http_only(true).finish();

        Ok(HttpResponse::Ok().cookie(cookie).json(TokenResponse {
            ok: true,
            access_token,
        }))
    }
}

async fn refresh_token(req: HttpRequest, pool: web::Data<Pool>) -> Result<HttpResponse, AppError> {
    let token = req.cookie("jid");
    match token {
        None => Ok(HttpResponse::BadRequest().json(TokenResponse {
            ok: false,
            access_token: "".to_string(),
        })),
        Some(s) => {
            let key = REFRESH_TOKEN_SECRET.clone();
            let data = key.verify_token::<ValidatedUser>(s.value(), None);
            match data {
                Err(e) => Ok(HttpResponse::BadRequest().json(TokenResponse {
                    ok: false,
                    access_token: "".to_string(),
                })),
                Ok(t) => {
                    let user = web::block(move || {
                        let conn = &pool.get().unwrap();
                        let user_key = UserKey::ID(t.custom.user_id);
                        models::find_user(conn, user_key)
                    })
                    .await?;
                    Ok(HttpResponse::Ok().json(TokenResponse {
                        ok: true,
                        access_token: auth::generate_access_token(&user)?,
                    }))
                }
            }
        }
    }
}

async fn dupsko(user: ValidatedUser) -> Result<HttpResponse, AppError> {
    Ok(HttpResponse::Ok().json(format!("sraken pierdaken: {}", user.user_id)))
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/register").route(web::post().to(register_user)))
        .service(web::resource("/login").route(web::post().to(login_user)))
        .service(web::resource("/dupsko").route(web::get().to(dupsko)))
        .service(web::resource("/refresh_token").route(web::get().to(refresh_token)));
}
