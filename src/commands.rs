use crate::db::{self, SqliteConnection};
use crate::models::*;
use chrono::{Local, NaiveDate};

const DEFAULT_FREQ_DAYS: i32 = 100;
const MAX_FREQ_DAYS: i32 = 365;
const DATE_FORMAT: &str = "%Y-%m-%d";

pub fn list_friends(conn: &SqliteConnection) {
    let results = db::load_all_friends(conn).expect("Error getting friends");
    for friend in results {
        println!("{}", friend);
    }
}

pub fn show_friend(name: String, conn: &SqliteConnection) {
    let friend = db::load_friend(&name, conn).expect("Error getting friend");
    println!("{}", friend);
}

pub fn add_friend(name: String, freq_days: Option<i32>, conn: &SqliteConnection) {
    let new_friend = NewFriend {
        name: name.clone(),
        freq_days: freq_days.unwrap_or(DEFAULT_FREQ_DAYS),
    };

    db::insert_friend(new_friend, conn).expect("Error adding friend");
    show_friend(name, conn);
}

pub fn remove_friend(name: String, conn: &SqliteConnection) {
    show_friend(name.clone(), conn);
    db::delete_friend(&name, conn).expect("Error removing friend");
}

pub fn set_frequency(name: String, freq_days: i32, conn: &SqliteConnection) {
    if freq_days <= 0 || freq_days > MAX_FREQ_DAYS {
        panic!(
            "Must see friends between every 1 day and every {} days",
            MAX_FREQ_DAYS
        );
    }

    db::update_freq_days(&name, freq_days, conn).expect("Error setting frequency");
    show_friend(name, conn);
}

pub fn record_seen(name: String, date: String, conn: &SqliteConnection) {
    let new_date = parse_date(date);

    let last_date = db::get_last_seen(&name, conn).expect("Error getting previously seen");
    if last_date.is_some() {
        let last_date = parse_date(last_date.unwrap());
        if last_date > new_date {
            panic!("Already seen more recently on {}", last_date);
        }
    }

    let local_date = Local::now().date().naive_local();
    if new_date > local_date {
        panic!("Cannot be seen in the future");
    }

    db::update_last_seen(&name, new_date.to_string(), conn).expect("Error recording seen");
    show_friend(name, conn);
}

fn parse_date(date: String) -> NaiveDate {
    NaiveDate::parse_from_str(&date, DATE_FORMAT).expect(&format!(
        "Date {} does not have format {}",
        date, DATE_FORMAT
    ))
}
