use crate::errors::AppError;
use crate::schema::users;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};
use diesel::prelude::*;

type Result<T> = std::result::Result<T, AppError>;

#[derive(Queryable, Identifiable, Serialize, Debug)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
}

pub fn create_user(conn: &PgConnection, username: &str, password: &str) -> Result<User> {
    let salt = SaltString::generate(&mut OsRng);
    let hashed_password = Argon2::default()
        .hash_password(password.as_bytes(), &salt)?
        .to_string();

    let user = diesel::insert_into(users::table)
        .values((
            users::username.eq(username),
            users::password.eq(hashed_password),
        ))
        .get_result(conn)?;
    Ok(user)
}

pub fn find_user(conn: &PgConnection, username: &str) -> Result<User> {
    users::table
        .filter(users::username.eq(username))
        .select((users::id, users::username, users::password))
        .first::<User>(conn)
        .map_err(Into::into)
}
