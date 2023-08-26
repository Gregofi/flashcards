
use diesel::sqlite::SqliteConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

mod models;
mod lib;
mod schema;
mod markdown;

pub fn estabilish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url).unwrap_or_else(|_|
        panic!("Error connecting to {}", database_url)
    )
}

fn main() {
    let db = &mut estabilish_connection();

    println!("Hello, world!");

}
