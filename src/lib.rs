pub mod db_operations;
pub mod models;
pub mod schema;

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
