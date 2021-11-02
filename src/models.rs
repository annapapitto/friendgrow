use crate::dates;
use crate::schema::friends;
use chrono::{Duration, NaiveDate};
use std::fmt;

#[derive(Identifiable, Queryable, Clone, Hash, PartialEq, Eq)]
pub struct Friend {
    pub id: i32,
    pub name: String,
    pub location: String,
    pub freq_weeks: i32,
    pub last_seen: Option<String>,
}

impl Friend {
    pub fn days_until_due(&self, today: NaiveDate) -> Option<i64> {
        self.last_seen.as_ref().map(|last_seen| {
            let last_seen = dates::parse_date(&last_seen);
            let weeks_to_next = Duration::weeks(self.freq_weeks as i64);
            let next_due = last_seen
                .checked_add_signed(weeks_to_next)
                .expect("Error calculating when to next see");
            (next_due - today).num_days()
        })
    }
}

impl fmt::Display for Friend {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let seen_str: String = self
            .last_seen
            .clone()
            .map_or("Never seen".to_string(), |last| {
                format!("Last seen on {}", last)
            });
        write!(
            f,
            "{}\tLocated in {}\tEvery {} weeks\t{}",
            self.name, self.location, self.freq_weeks, seen_str
        )
    }
}

#[derive(Insertable)]
#[table_name = "friends"]
pub struct NewFriend {
    pub name: String,
    pub location: String,
    pub freq_weeks: i32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_days_until_due_never_seen() {
        let today = NaiveDate::from_ymd(2021, 04, 02);
        let friend = Friend {
            id: 1,
            name: "Test".to_string(),
            location: "Nowhere".to_string(),
            freq_weeks: 2,
            last_seen: None,
        };

        assert_eq!(friend.days_until_due(today), None);
    }

    #[test]
    fn test_get_days_until_due_up_soon() {
        let today = NaiveDate::from_ymd(2021, 04, 02);
        let friend = Friend {
            id: 1,
            name: "Test".to_string(),
            location: "Nowhere".to_string(),
            freq_weeks: 2,
            last_seen: Some("2021-04-01".to_string()),
        };

        assert_eq!(friend.days_until_due(today), Some(13));
    }

    #[test]
    fn test_get_days_until_due_overdue() {
        let today = NaiveDate::from_ymd(2021, 04, 20);
        let friend = Friend {
            id: 1,
            name: "Test".to_string(),
            location: "Nowhere".to_string(),
            freq_weeks: 2,
            last_seen: Some("2021-04-01".to_string()),
        };

        assert_eq!(friend.days_until_due(today), Some(-5));
    }
}
