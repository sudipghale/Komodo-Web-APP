extern crate web_api;

use self::web_api::*;
use std::io::{stdin, Read};

fn main() {
    let connection = establish_connection();

    println!("What would you like your username to be?");
    let mut username = String::new();
    stdin().read_line(&mut username).unwrap();
    let username = &username[..(username.len() - 1)]; // Drop the newline character

    let password = "fancystrongsuperpassword@123";
    let email = "ashish.mainali@ymail.com";

    let user = create_account(&connection, &email, &username, &password);
    println!(
        "\nSaved user {} with password {} and email {} and id {}",
        username, password, email, user.id
    );
}
