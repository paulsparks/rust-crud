use rust_crud::*;
use std::io::{self, stdin};

fn main() -> io::Result<()> {
    println!("Do you already have an account? (Y/n)");

    let mut selection = String::new();
    stdin().read_line(&mut selection)?;
    let selection = selection.trim_end();

    let has_account = selection != "n";

    if !has_account {
        println!("Enter username and password for new account:")
    };
    let (username, password) = prompt_login()?;

    Ok(())
}
