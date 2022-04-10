use actix_web::{cookie::Cookie, web, HttpResponse};
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use diesel::prelude::*;
use jwt_simple::prelude::Duration;

use crate::auth::{self, ValidatedUser};
use crate::errors::AppError;
use crate::schema::users;
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
    Ok(HttpResponse::Ok()
        .cookie(cookie)
        .json(LoginResponse { access_token }))
}

#[derive(Debug, Serialize)]
struct LoginResponse {
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
        models::find_user(conn, &username)
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

        Ok(HttpResponse::Ok()
            .cookie(cookie)
            .json(LoginResponse { access_token }))
    }
}

async fn dupsko(user: ValidatedUser) -> Result<HttpResponse, AppError> {
    Ok(HttpResponse::Ok().json("sraken pierdaken"))
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/register").route(web::post().to(register_user)))
        .service(web::resource("/login").route(web::post().to(login_user)))
        .service(web::resource("/dupsko").route(web::get().to(dupsko)));
}
