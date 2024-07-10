pub mod models;
pub mod schema;

use diesel::{pg::PgConnection, Connection, RunQueryDsl};
use models::{NewUser, User};
use std::{
    env,
    io::{self, stdin, stdout, Write},
};

pub fn establish_connection() -> PgConnection {
    let connection_string =
        env::var("DATABASE_URL").expect("Environment variable DATABASE_URL must be set");

    PgConnection::establish(&connection_string)
        .unwrap_or_else(|err| panic!("Database error: {}", err))
}

pub fn create_user(conn: &mut PgConnection, username: &str, password: &str) -> User {
    use crate::schema::users;

    let new_user = NewUser { username, password };

    diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(conn)
        .unwrap_or_else(|err| panic!("Error creating user: {}", err))
}

pub fn prompt_login() -> Result<(String, String), io::Error> {
    let mut username = String::new();
    let mut password = String::new();

    print!("username: ");
    stdout().flush()?;
    stdin().read_line(&mut username)?;
    print!("password: ");
    stdout().flush()?;
    stdin().read_line(&mut password)?;

    username = username.trim_end().to_string();
    password = password.trim_end().to_string();

    Ok((username, password))
}
