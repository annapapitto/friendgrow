use crate::models::*;
use crate::schema::friends::{self, dsl::*};
use anyhow::{Context, Result};
pub use diesel::prelude::SqliteConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

const DB_KEY: &str = "DATABASE_URL";

pub fn connect() -> Result<SqliteConnection> {
    dotenv().ok();

    let database_url = env::var(DB_KEY).context("DATABASE_URL must be set in the environment")?;
    SqliteConnection::establish(&database_url).context("Failed to establish connection to database")
}

pub fn load_all_friends(conn: &SqliteConnection) -> QueryResult<Vec<Friend>> {
    friends::table.order_by(freq_weeks).load::<Friend>(conn)
}

pub fn load_friend(friend_name: &String, conn: &SqliteConnection) -> QueryResult<Friend> {
    friends
        .filter(name.eq(friend_name.clone()))
        .first::<Friend>(conn)
}

pub fn insert_friend(new_friend: NewFriend, conn: &SqliteConnection) -> QueryResult<usize> {
    diesel::insert_into(friends::table)
        .values(&new_friend)
        .execute(conn)
}

pub fn delete_friend(friend_name: &String, conn: &SqliteConnection) -> QueryResult<usize> {
    diesel::delete(friends.filter(name.eq(friend_name.clone()))).execute(conn)
}

pub fn update_freq_weeks(
    friend_name: &String,
    new_freq_weeks: i32,
    conn: &SqliteConnection,
) -> QueryResult<usize> {
    let friend = load_friend(&friend_name, conn)?;
    diesel::update(&friend)
        .set(freq_weeks.eq(new_freq_weeks))
        .execute(conn)
}

pub fn update_name(
    friend_name: &String,
    new_name: &String,
    conn: &SqliteConnection,
) -> QueryResult<usize> {
    let friend = load_friend(&friend_name, conn)?;
    diesel::update(&friend).set(name.eq(new_name)).execute(conn)
}

pub fn update_location(
    friend_name: &String,
    new_location: String,
    conn: &SqliteConnection,
) -> QueryResult<usize> {
    let friend = load_friend(&friend_name, conn)?;
    diesel::update(&friend)
        .set(location.eq(new_location))
        .execute(conn)
}

pub fn update_last_seen(
    friend_name: &String,
    new_last_seen: String,
    conn: &SqliteConnection,
) -> QueryResult<usize> {
    let seen_friend = load_friend(&friend_name, conn)?;
    diesel::update(&seen_friend)
        .set(last_seen.eq(new_last_seen))
        .execute(conn)
}
