use crate::models::*;
use chrono::{Duration, Local, NaiveDate};
use core::panic;
use priority_queue::PriorityQueue;

const MAX_FREQ_WEEKS: i32 = 52;
const DATE_FORMAT: &str = "%Y-%m-%d";
const UP_SOON_CUTOFF_DAYS: i64 = 20;

pub struct UpcomingFriends {
    never_seen: Vec<Friend>,
    overdue: FriendQueue,
    up_soon: FriendQueue,
}

impl UpcomingFriends {
    pub fn new() -> Self {
        Self {
            never_seen: Vec::new(),
            overdue: FriendQueue::new(true),
            up_soon: FriendQueue::new(false),
        }
    }

    pub fn push_seen(&mut self, friend: Friend, days_until_due: i64) {
        match days_until_due {
            d if d < 0 => {
                self.overdue.push(friend.clone(), d);
            }
            d if d < UP_SOON_CUTOFF_DAYS => {
                self.up_soon.push(friend.clone(), d);
            }
            _ => {}
        };
    }

    pub fn push_never_seen(&mut self, friend: Friend) {
        self.never_seen.push(friend.clone());
    }

    pub fn print(&self) {
        if !self.never_seen.is_empty() {
            println!("\nNEVER SEEN");
            for friend in &self.never_seen {
                println!("{}", friend);
            }
        }
        if !self.overdue.is_empty() {
            println!("\nOVERDUE");
            self.overdue.print_due_days();
        }
        if !self.up_soon.is_empty() {
            println!("\nUP SOON");
            self.up_soon.print_due_days();
        }
    }
}

struct FriendQueue {
    overdue: bool,
    queue_by_days: PriorityQueue<Friend, i64>,
}

impl FriendQueue {
    fn new(overdue: bool) -> Self {
        Self {
            overdue,
            queue_by_days: PriorityQueue::new(),
        }
    }

    fn push(&mut self, friend: Friend, days_until_due: i64) {
        match self.overdue {
            true => {
                if days_until_due >= 0 {
                    panic!("Cannot be overdue with a non-negative days until due");
                }
                self.queue_by_days.push(friend, -days_until_due);
            }
            false => {
                if days_until_due < 0 {
                    panic!("Must be overdue with a negative days until due");
                }
                self.queue_by_days.push(friend, -days_until_due);
            }
        }
    }

    fn is_empty(&self) -> bool {
        self.queue_by_days.is_empty()
    }

    fn print_due_days(&self) {
        for (friend, due_in_days) in self.queue_by_days.clone().into_sorted_iter() {
            let due_on = match self.overdue {
                true => {
                    format!("Due {} days ago", due_in_days)
                }
                false => {
                    format!("Due in {} days", -due_in_days)
                }
            };
            println!("{}\t{}", due_on, friend);
        }
    }
}

pub fn get_days_until_due(last_seen: &str, freq_weeks: i32, today: NaiveDate) -> i64 {
    let last_seen = parse_date(last_seen);
    let weeks_to_next = Duration::weeks(freq_weeks as i64);
    let next_due = last_seen
        .checked_add_signed(weeks_to_next)
        .expect("Error calculating when to next see");
    (next_due - today).num_days()
}

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
mod private_tests {
    use super::*;

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
