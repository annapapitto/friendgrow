use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

const DB_KEY: &str = "DATABASE_URL";

pub fn connect() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var(DB_KEY).expect(&format!("{} must be set", DB_KEY));
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}
