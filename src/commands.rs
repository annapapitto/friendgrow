use crate::db::{self, SqliteConnection};
use crate::models::*;
use chrono::{Local, NaiveDate};

const DEFAULT_FREQ_WEEKS: i32 = 10;
const MAX_FREQ_WEEKS: i32 = 52;
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

pub fn add_friend(name: String, freq_weeks: Option<i32>, conn: &SqliteConnection) {
    let new_friend = NewFriend {
        name: name.clone(),
        freq_weeks: freq_weeks.unwrap_or(DEFAULT_FREQ_WEEKS),
    };

    db::insert_friend(new_friend, conn).expect("Error adding friend");
    show_friend(name, conn);
}

pub fn remove_friend(name: String, conn: &SqliteConnection) {
    show_friend(name.clone(), conn);
    db::delete_friend(&name, conn).expect("Error removing friend");
}

pub fn set_frequency(name: String, freq_weeks: i32, conn: &SqliteConnection) {
    check_frequency(freq_weeks);

    db::update_freq_weeks(&name, freq_weeks, conn).expect("Error setting frequency");
    show_friend(name, conn);
}

pub fn record_seen(name: String, date: String, conn: &SqliteConnection) {
    let new_date = parse_date(&date);

    let last_date = db::get_last_seen(&name, conn).expect("Error getting previously seen");
    check_new_seen(new_date, last_date);

    db::update_last_seen(&name, new_date.to_string(), conn).expect("Error recording seen");
    show_friend(name, conn);
}

fn parse_date(date: &str) -> NaiveDate {
    NaiveDate::parse_from_str(date, DATE_FORMAT).expect(&format!(
        "Date {} does not have format {}",
        date, DATE_FORMAT
    ))
}

fn check_frequency(freq_weeks: i32) {
    if freq_weeks <= 0 || freq_weeks > MAX_FREQ_WEEKS {
        panic!(
            "Must see friends between every 1 week and every {} weeks",
            MAX_FREQ_WEEKS
        );
    }
}

fn check_new_seen(new_date: NaiveDate, last_date: Option<String>) {
    if last_date.is_some() {
        let last_date = parse_date(&last_date.unwrap());
        if last_date > new_date {
            panic!("Already seen more recently on {}", last_date);
        }
    }

    if new_date > local_today() {
        panic!("Cannot record in the future");
    }
}

fn local_today() -> NaiveDate {
    Local::now().date().naive_local()
}

#[cfg(test)]
mod private_tests {
    use super::*;
    use chrono::Duration;

    #[test]
    fn test_parse_date() {
        let correct = NaiveDate::from_ymd(2021, 10, 26);
        let res = parse_date("2021-10-26");
        assert_eq!(res, correct);

        let correct = NaiveDate::from_ymd(100, 2, 3);
        let res = parse_date("100-2-3");
        assert_eq!(res, correct);
        let res = parse_date("0100-02-03");
        assert_eq!(res, correct);
    }

    #[test]
    #[should_panic]
    fn test_parse_date_empty() {
        parse_date("");
    }

    #[test]
    #[should_panic]
    fn test_parse_date_month_day_flipped() {
        parse_date("2021-20-10");
    }

    #[test]
    fn test_check_frequency() {
        check_frequency(1);
        check_frequency(20);
        check_frequency(52);
    }

    #[test]
    #[should_panic]
    fn test_check_frequency_zero() {
        check_frequency(0);
    }

    #[test]
    #[should_panic]
    fn test_check_frequency_negative() {
        check_frequency(-1);
    }

    #[test]
    #[should_panic]
    fn test_check_frequency_big() {
        check_frequency(53);
    }

    #[test]
    fn test_check_new_seen() {
        let new_date = NaiveDate::from_ymd(102, 2, 5);
        check_new_seen(new_date, Some("100-2-4".to_string()));
        check_new_seen(new_date, Some("101-12-3".to_string()));
        check_new_seen(new_date, None);
    }

    #[test]
    #[should_panic]
    fn test_check_new_seen_earlier() {
        let new_date = NaiveDate::from_ymd(200, 4, 7);
        check_new_seen(new_date, Some("200-4-8".to_string()));
    }

    #[test]
    #[should_panic]
    fn test_check_new_seen_future() {
        let tomorrow = local_today() + Duration::days(1);
        check_new_seen(tomorrow, None);
    }
}
