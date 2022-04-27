use actix_web::{cookie::Cookie, web, HttpResponse};
use actix_web::{HttpMessage, HttpRequest};
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use jwt_simple::prelude::*;

use crate::auth::{self, ValidatedUser};
use crate::errors::AppError;
//use crate::models::users::UserKey;
use crate::models;
use crate::prisma::Role;
use crate::REFRESH_TOKEN_SECRET;

#[derive(Debug, Deserialize, Serialize)]
struct UserRegisterInput {
    email: String,
    password: String,
    username: String,
    role: Role,
}

#[derive(Debug, Serialize)]
struct TokenResponse {
    ok: bool,
    access_token: String,
}

async fn register_user(
    credentials: web::Json<UserRegisterInput>,
    conn: web::Data<crate::DbClient>,
) -> Result<HttpResponse, AppError> {
    let credentials = credentials.into_inner();
    let username = credentials.username;
    let password = credentials.password;
    let email = credentials.email;
    let role = credentials.role;

    let conn = conn.client.lock().unwrap();
    let user = models::users::create_user(&conn, email, username, password, role).await?;

    let access_token = auth::generate_access_token(&user)?;
    let refresh_token = auth::generate_refresh_token(&user)?;
    let cookie = Cookie::build("jid", refresh_token).http_only(true).finish();
    Ok(HttpResponse::Ok().cookie(cookie).json(TokenResponse {
        ok: true,
        access_token,
    }))
}

//#[derive(Debug, Deserialize, Serialize)]
//struct UserLoginInput {
//    email: String,
//    password: String,
//}
//
//async fn login_user(
//    data: web::Json<UserLoginInput>,
//    pool: web::Data<Pool>,
//) -> Result<HttpResponse, AppError> {
//    let credentials = data.into_inner();
//    let email = credentials.email;
//    let password = credentials.password;
//    let user = web::block(move || {
//        let conn = &mut pool.get().unwrap();
//        let user_key = UserKey::Email(email.as_str());
//        models::users::find_user(conn, user_key)
//    })
//    .await?;
//
//    let hash = PasswordHash::new(&user.password)?;
//    let is_correct = Argon2::default()
//        .verify_password(password.as_bytes(), &hash)
//        .is_ok();
//    if !is_correct {
//        Err(AppError::BadPassword)
//    } else {
//        let access_token = auth::generate_access_token(&user)?;
//        let refresh_token = auth::generate_refresh_token(&user)?;
//        let cookie = auth::create_refresh_token_cookie(&refresh_token);
//
//        Ok(HttpResponse::Ok().cookie(cookie).json(TokenResponse {
//            ok: true,
//            access_token,
//        }))
//    }
//}
//
//async fn refresh_token(req: HttpRequest, pool: web::Data<Pool>) -> Result<HttpResponse, AppError> {
//    let token = req.cookie("jid");
//    match token {
//        None => Ok(HttpResponse::BadRequest().json(TokenResponse {
//            ok: false,
//            access_token: "".to_string(),
//        })),
//        Some(s) => {
//            let key = REFRESH_TOKEN_SECRET.clone();
//            let data = key.verify_token::<auth::RefreshTokenClaims>(s.value(), None);
//            match data {
//                Err(_) => Ok(HttpResponse::BadRequest().json(TokenResponse {
//                    ok: false,
//                    access_token: "".to_string(),
//                })),
//                Ok(t) => {
//                    let user = web::block(move || {
//                        let conn = &mut pool.get().unwrap();
//                        let user_key = UserKey::ID(t.custom.user_id);
//                        models::users::find_user(conn, user_key)
//                    })
//                    .await?;
//
//                    if t.custom.token_version != user.token_version {
//                        return Ok(HttpResponse::BadRequest().json(TokenResponse {
//                            ok: false,
//                            access_token: "".to_string(),
//                        }));
//                    }
//
//                    let refresh_token = auth::generate_refresh_token(&user)?;
//                    let access_token = auth::generate_access_token(&user)?;
//                    let cookie = auth::create_refresh_token_cookie(&refresh_token);
//                    Ok(HttpResponse::Ok().cookie(cookie).json(TokenResponse {
//                        ok: true,
//                        access_token,
//                    }))
//                }
//            }
//        }
//    }
//}
//
//async fn logout_user(_user: ValidatedUser) -> Result<HttpResponse, AppError> {
//    let access_token = String::from("");
//    let cookie = auth::create_refresh_token_cookie("");
//    Ok(HttpResponse::Ok().cookie(cookie).json(TokenResponse {
//        ok: true,
//        access_token,
//    }))
//}
//
//async fn ok() -> HttpResponse {
//    HttpResponse::Ok().finish()
//}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/register").route(web::post().to(register_user)));
    //.service(web::resource("/login").route(web::post().to(login_user)))
    //.service(web::resource("/refresh_token").route(web::get().to(refresh_token)))
    //.service(web::resource("/logout").route(web::post().to(logout_user)))
    //.service(web::resource("/").route(web::get().to(ok)));
}
