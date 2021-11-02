use chrono::{Local, NaiveDate};
use core::panic;

const MAX_FREQ_WEEKS: i32 = 52;
const DATE_FORMAT: &str = "%Y-%m-%d";

pub fn parse_date(date: &str) -> NaiveDate {
    NaiveDate::parse_from_str(date, DATE_FORMAT).expect(&format!(
        "Date {} does not have format {}",
        date, DATE_FORMAT
    ))
}

pub fn check_frequency(freq_weeks: i32) {
    if freq_weeks <= 0 || freq_weeks > MAX_FREQ_WEEKS {
        panic!(
            "Must see friends between every 1 week and every {} weeks",
            MAX_FREQ_WEEKS
        );
    }
}

pub fn check_new_seen(new_date: NaiveDate, last_date: Option<String>) {
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

pub fn local_today() -> NaiveDate {
    Local::now().date().naive_local()
}

#[cfg(test)]
mod tests {
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
