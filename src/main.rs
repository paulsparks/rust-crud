use rust_crud::*;
use std::io::{self, stdin};

fn main() -> io::Result<()> {
    println!("Do you already have an account? (Y/n)");

    let mut selection = String::new();
    stdin().read_line(&mut selection)?;
    let selection = selection.trim_end();

    let has_account = selection != "n";

    let connection = &mut establish_connection();

    let user;

    if !has_account {
        println!("Enter username and password for new account:");

        let (username, password) = prompt_login()?;

        user = create_user(connection, &username, &password);
        println!("Created account for {}", user.username);

        return Ok(());
    }

    loop {
        let (username, password) = prompt_login()?;

        match get_user(connection, &username, &password) {
            Ok(val) => {
                user = val;
                break;
            }
            Err(_) => println!("\nUser not found. Try again!"),
        }
    }

    println!("\nWelcome {}!", user.username);

    Ok(())
}
