use crate::models::*;
use crate::schema::friends::{self, dsl::*};
pub use diesel::prelude::SqliteConnection;
use diesel::prelude::*;
use diesel::result::Error;
use dotenv::dotenv;
use std::env;

const DB_KEY: &str = "DATABASE_URL";

pub fn connect() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var(DB_KEY).expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn load_all_friends(conn: &SqliteConnection) -> Result<Vec<Friend>, Error> {
    friends::table.load::<Friend>(conn)
}

pub fn load_friend(friend_name: &String, conn: &SqliteConnection) -> Friend {
    friends
        .filter(name.eq(friend_name.clone()))
        .first::<Friend>(conn)
        .expect(&format!("No friend named {}", friend_name))
}

pub fn insert_friend(new_friend: NewFriend, conn: &SqliteConnection) -> Result<usize, Error> {
    diesel::insert_into(friends::table)
        .values(&new_friend)
        .execute(conn)
}

pub fn update_last_seen(
    friend_name: String,
    new_last_seen: String,
    conn: &SqliteConnection,
) -> Result<usize, Error> {
    let seen_friend = load_friend(&friend_name, conn);
    diesel::update(&seen_friend)
        .set(last_seen.eq(new_last_seen))
        .execute(conn)
}
