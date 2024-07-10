use rust_crud::{db_operations::*, prompt_login};
use std::{
    io::{self, stdin},
    process,
};

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

        user = create_user(connection, &username, &password).unwrap_or_else(|_| {
            eprintln!("Error creating user. Username cannot exceed 20 characters.");
            process::exit(1);
        });
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
            Err(diesel::result::Error::NotFound) => println!("\nUser not found. Try again!"),
            Err(_) => eprintln!("\nDatabase error"),
        }
    }

    println!("\nWelcome {}!", user.username);

    Ok(())
}
