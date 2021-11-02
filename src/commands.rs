use crate::dates;
use crate::db::{self, SqliteConnection};
use crate::models::*;
use crate::upcoming::UpcomingFriends;

const DEFAULT_FREQ_WEEKS: i32 = 10;

pub fn list_friends(conn: &SqliteConnection) {
    let all_friends = db::load_all_friends(conn).expect("Error getting friends");
    for friend in all_friends {
        println!("{}", friend);
    }
}

pub fn show_friend(name: String, conn: &SqliteConnection) {
    let friend = db::load_friend(&name, conn).expect("Error getting friend");
    println!("{}", friend);
}

pub fn add_friend(
    name: String,
    location: String,
    freq_weeks: Option<i32>,
    conn: &SqliteConnection,
) {
    let new_friend = NewFriend {
        name: name.clone(),
        location: location.clone(),
        freq_weeks: freq_weeks.unwrap_or(DEFAULT_FREQ_WEEKS),
    };

    db::insert_friend(new_friend, conn).expect("Error adding friend");
    show_friend(name, conn);
}

pub fn remove_friend(name: String, conn: &SqliteConnection) {
    show_friend(name.clone(), conn);
    db::delete_friend(&name, conn).expect("Error removing friend");
}

pub fn set_location(name: String, location: String, conn: &SqliteConnection) {
    db::update_location(&name, location, conn).expect("Error setting location");
    show_friend(name, conn);
}

pub fn set_frequency(name: String, freq_weeks: i32, conn: &SqliteConnection) {
    dates::check_frequency(freq_weeks);

    db::update_freq_weeks(&name, freq_weeks, conn).expect("Error setting frequency");
    show_friend(name, conn);
}

pub fn record_seen(name: String, date: String, conn: &SqliteConnection) {
    let new_date = dates::parse_date(&date);

    let friend = db::load_friend(&name, conn).expect("Error getting friend");
    dates::check_new_seen(new_date, friend.last_seen);

    db::update_last_seen(&name, new_date.to_string(), conn).expect("Error recording seen");
    show_friend(name, conn);
}

pub fn list_upcoming(conn: &SqliteConnection) {
    let results = db::load_all_friends(conn).expect("Error getting friends");
    let today = dates::local_today();
    let mut upcoming_friends = UpcomingFriends::new();

    for friend in results {
        upcoming_friends.push(friend.clone(), today);
    }

    upcoming_friends.print();
}
