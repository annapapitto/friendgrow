use crate::models::*;
use anyhow::Result;
use chrono::NaiveDate;
use core::panic;
use prettytable::{format, Row, Table};
use priority_queue::PriorityQueue;

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

    pub fn push(&mut self, friend: Friend, today: NaiveDate) -> Result<()> {
        match friend.days_until_due(today) {
            Ok(Some(days_until_due)) => Ok(self.push_seen(friend, days_until_due)),
            Ok(None) => Ok(self.push_never_seen(friend)),
            Err(e) => Err(e),
        }
    }

    fn push_seen(&mut self, friend: Friend, days_until_due: i64) {
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

    fn push_never_seen(&mut self, friend: Friend) {
        self.never_seen.push(friend.clone());
    }

    pub fn print(&self) {
        let mut table = Table::new();
        table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
        table.set_titles(Friend::get_table_titles(true));

        for friend in &self.never_seen {
            table.add_row(friend.get_table_row());
        }
        for r in self.overdue.get_table_rows() {
            table.add_row(r);
        }
        for r in self.up_soon.get_table_rows() {
            table.add_row(r);
        }

        table.printstd();
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

    fn get_table_rows(&self) -> Vec<Row> {
        let mut rows = vec![];
        if self.is_empty() {
            return rows;
        }

        for (friend, due_in_days) in self.queue_by_days.clone().into_sorted_iter() {
            let due_on = match self.overdue {
                true => {
                    format!("{} days ago", due_in_days)
                }
                false => {
                    format!("In {} days", -due_in_days)
                }
            };
            rows.push(friend.get_table_row_with_due(&due_on));
        }
        rows
    }
}
