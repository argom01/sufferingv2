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

pub fn create_user(conn: &MysqlConnection, username: &str, password: &str) -> Result<User> {
    let salt = SaltString::generate(&mut OsRng);
    let hashed_password = Argon2::default()
        .hash_password(password.as_bytes(), &salt)?
        .to_string();

    conn.transaction(|| {
        diesel::insert_into(users::table)
        .values((
            users::username.eq(username),
            users::password.eq(hashed_password)
        ))
        .execute(conn)?;

        users::table
        .order(users::id.desc())
        .select((users::id, users::username, users::password))
        .first(conn)
        .map_err(Into::into)
    })
}

pub enum UserKey<'a> {
    Username(&'a str),
    ID(i32),
}

pub fn find_user<'a>(conn: &MysqlConnection, key: UserKey<'a>) -> Result<User> {
    match key {
        UserKey::Username(name) => users::table
            .filter(users::username.eq(name))
            .select((users::id, users::username, users::password))
            .first::<User>(conn)
            .map_err(Into::into),

        UserKey::ID(ID) => users::table
            .filter(users::id.eq(ID))
            .select((users::id, users::username, users::password))
            .first::<User>(conn)
            .map_err(Into::into),
    }
}
