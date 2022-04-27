use crate::{errors::AppError, models::Result};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};

use crate::prisma::PrismaClient;
use crate::prisma::{user, Role};

pub async fn create_user(
    conn: &PrismaClient,
    email: String,
    username: String,
    password: String,
    role: Role,
) -> Result<user::Data> {
    let salt = SaltString::generate(&mut OsRng);
    let hashed_password = Argon2::default()
        .hash_password(password.as_bytes(), &salt)?
        .to_string();
    let user: user::Data = conn
        .user()
        .create(
            user::email::set(email),
            user::username::set(username),
            user::password::set(hashed_password),
            user::role::set(role),
            vec![],
        )
        .exec()
        .await?;
    Ok(user)
}

pub enum UserKey<'a> {
    Email(&'a str),
    ID(i32),
}

pub async fn find_user<'a>(conn: &PrismaClient, key: UserKey<'a>) -> Result<user::Data> {
    match key {
        UserKey::Email(email) => {
            let user = conn
                .user()
                .find_unique(user::email::equals(email.to_string()))
                .exec()
                .await?;
            match user {
                Some(s) => Ok(s),
                None => Err(AppError::RecordNotFound),
            }
        }

        UserKey::ID(id) => {
            let user = conn.user().find_unique(user::id::equals(id)).exec().await?;
            match user {
                Some(s) => Ok(s),
                None => Err(AppError::RecordNotFound),
            }
        }
    }
}

pub async fn revoke_refresh_tokens_for_user(conn: &PrismaClient, id: i32) -> Result<()> {
    conn.user()
        .find_unique(user::id::equals(id))
        .update(vec![user::token_version::increment(1)])
        .exec()
        .await?;
    Ok(())
}
