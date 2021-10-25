use crate::db::{self, SqliteConnection};
use crate::models::*;
use chrono::{Local, NaiveDate};

const DEFAULT_FREQ_DAYS: i32 = 100;
const DATE_FORMAT: &str = "%Y-%m-%d";

pub fn list_friends(conn: &SqliteConnection) {
    let results = db::load_all_friends(conn).expect("Error getting friends");
    for friend in results {
        println!("{}", friend);
    }
}

pub fn show_friend(name: String, conn: &SqliteConnection) {
    let friend = db::load_friend(&name, conn).expect(&format!("Error getting friend {}", name));
    println!("{}", friend);
}

pub fn add_friend(name: String, conn: &SqliteConnection) {
    let new_friend = NewFriend {
        name: name.clone(),
        freq_days: DEFAULT_FREQ_DAYS,
    };

    db::insert_friend(new_friend, conn).expect(&format!("Error adding friend {}", name));
    show_friend(name, conn);
}

pub fn record_seen(name: String, date: String, conn: &SqliteConnection) {
    let new_date = parse_date(date);

    let last_date =
        db::get_last_seen(&name, conn).expect(&format!("Error getting previously seen"));
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

    db::update_last_seen(&name, new_date.to_string(), conn)
        .expect(&format!("Error recording seen"));
    show_friend(name, conn);
}

fn parse_date(date: String) -> NaiveDate {
    NaiveDate::parse_from_str(&date, DATE_FORMAT).expect(&format!(
        "Date {} does not have format {}",
        date, DATE_FORMAT
    ))
}
