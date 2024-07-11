pub mod db_operations;
pub mod models;
pub mod routes;
pub mod schema;
use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, Pool},
};
use std::env;

use std::io::{self, stdin, stdout, Write};

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

pub fn get_connection_pool() -> Pool<ConnectionManager<PgConnection>> {
    let url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(&url);
    Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .expect("Could not build connection pool")
}
