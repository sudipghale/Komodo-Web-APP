#[macro_use]
extern crate diesel;
extern crate dotenv;

//pub mod models;
pub mod models;
pub mod schema;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

use self::models::{Account, NewAccount};

pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

pub fn create_account<'a>(
    conn: &PgConnection,
    email: &'a str,
    username: &'a str,
    password: &'a str,
) -> Account {
    use schema::account;

    let new_account = NewAccount {
        email: email,
        username: username,
        password: password,
    };

    diesel::insert_into(account::table)
        .values(&new_account)
        .get_result(conn)
        .expect("Error signing up new account.")
}
