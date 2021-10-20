use crate::db::*;
use crate::models::*;
use chrono::NaiveDate;

const DEFAULT_FREQ_DAYS: i32 = 100;
const DATE_FORMAT: &str = "%Y-%m-%d";

pub fn list_friends(conn: &SqliteConnection) {
    let results = load_all_friends(conn).expect("Error getting friends");
    for friend in results {
        println!("{}", friend);
    }
}

pub fn add_friend(name: String, conn: &SqliteConnection) {
    let new_friend = NewFriend {
        name: name.clone(),
        freq_days: DEFAULT_FREQ_DAYS,
    };
    insert_friend(new_friend, conn).expect(&format!("Error adding friend {}", name));
}

pub fn record_seen(name: String, date: String, conn: &SqliteConnection) {
    let date = NaiveDate::parse_from_str(&date, DATE_FORMAT).expect(&format!(
        "Date {} does not have format {}",
        date, DATE_FORMAT
    ));

    // TODO check that the new date is later than the existing date

    // TODO check that the new date is no earlier than today

    update_last_seen(name, date.to_string(), conn).expect(&format!("Error recording seen"));
}
