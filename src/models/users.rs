use crate::models::Result;
use crate::schema::users;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};
use diesel::prelude::*;

#[derive(Queryable, Identifiable, Serialize, Debug)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub password: String,
    pub username: String,
    pub token_version: i32,
}

pub fn create_user(
    conn: &mut MysqlConnection,
    email: &str,
    username: &str,
    password: &str,
    is_admin: bool,
) -> Result<User> {
    let salt = SaltString::generate(&mut OsRng);
    let hashed_password = Argon2::default()
        .hash_password(password.as_bytes(), &salt)?
        .to_string();

    conn.transaction(|conn| {
        diesel::insert_into(users::table)
            .values((
                users::username.eq(username),
                users::password.eq(hashed_password),
                users::email.eq(email),
                users::is_admin.eq(is_admin),
            ))
            .execute(conn)?;

        users::table
            .order(users::id.desc())
            .select((
                users::id,
                users::email,
                users::password,
                users::username,
                users::token_version,
            ))
            .first(conn)
            .map_err(Into::into)
    })
}

pub enum UserKey<'a> {
    Email(&'a str),
    ID(i32),
}

pub fn find_user<'a>(conn: &mut MysqlConnection, key: UserKey<'a>) -> Result<User> {
    match key {
        UserKey::Email(email) => users::table
            .filter(users::email.eq(email))
            .select((
                users::id,
                users::email,
                users::password,
                users::username,
                users::token_version,
            ))
            .first::<User>(conn)
            .map_err(Into::into),

        UserKey::ID(ID) => users::table
            .filter(users::id.eq(ID))
            .select((
                users::id,
                users::email,
                users::password,
                users::username,
                users::token_version,
            ))
            .first::<User>(conn)
            .map_err(Into::into),
    }
}

pub fn revoke_refresh_tokens_for_user(conn: &mut MysqlConnection, id: i32) -> Result<()> {
    diesel::update(users::table.filter(users::id.eq(id)))
        .set(users::token_version.eq(users::token_version + 1))
        .execute(conn)?;
    Ok(())
}
