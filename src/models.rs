use crate::dates;
use crate::schema::friends;
use anyhow::Result;
use chrono::{Duration, NaiveDate};
use prettytable::{Cell, Row};
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
    pub fn days_until_due(&self, today: NaiveDate) -> Result<Option<i64>> {
        if self.last_seen.is_none() {
            return Ok(None);
        }
        let last_seen = dates::parse_date(&self.last_seen.clone().unwrap())?;
        let weeks_to_next = Duration::weeks(self.freq_weeks as i64);
        let next_due = last_seen + weeks_to_next;
        Ok(Some((next_due - today).num_days()))
    }

    pub fn get_table_row(&self) -> Row {
        row![
            self.name,
            self.location,
            format!("{} weeks", self.freq_weeks),
            self.last_seen.clone().unwrap_or("Never".to_string()),
        ]
    }

    pub fn get_table_row_with_due(&self, due_on: &str) -> Row {
        let mut r = self.get_table_row();
        r.add_cell(Cell::new(due_on));
        r
    }

    pub fn get_table_titles(with_due: bool) -> Row {
        let mut r = row!["Name", "Location", "Frequency", "Last seen"];
        if with_due {
            r.add_cell(Cell::new("Due"));
        }
        r
    }
}

impl fmt::Display for Friend {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut seen_str: String = self
            .last_seen
            .clone()
            .map_or("not seen yet".to_string(), |last| {
                format!("last seen on {}", last)
            });

        let days_until_due = self
            .days_until_due(dates::local_today())
            .map_err(|_| fmt::Error)?;
        let next_due = Self::display_due_on(days_until_due);
        next_due.map(|n| seen_str.push_str(&format!(", see next {}", n)));

        write!(
            f,
            "{} ({}) every {} weeks, {}",
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

        assert_eq!(friend.days_until_due(today).unwrap(), None);
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

        assert_eq!(friend.days_until_due(today).unwrap(), Some(13));
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

        assert_eq!(friend.days_until_due(today).unwrap(), Some(-5));
    }
}
