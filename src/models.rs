use crate::schema::friends;
use std::fmt;

#[derive(Identifiable, Queryable)]
pub struct Friend {
    pub id: i32,
    pub name: String,
    pub freq_weeks: i32,
    pub last_seen: Option<String>,
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
            "{}\tEvery {} weeks\t{}",
            self.name, self.freq_weeks, seen_str
        )
    }
}

#[derive(Insertable)]
#[table_name = "friends"]
pub struct NewFriend {
    pub name: String,
    pub freq_weeks: i32,
}
