use crate::models::*;
use anyhow::Result;
use chrono::NaiveDate;
use prettytable::{format, Table};
use priority_queue::PriorityQueue;
use std::cmp::Ordering;

const UP_SOON_CUTOFF_DAYS: u16 = 15;

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum DueDays {
    NotSeen,
    OverDue(u16),
    DueIn(u16),
}

impl Ord for DueDays {
    fn cmp(&self, other: &Self) -> Ordering {
        use DueDays::*;

        match self {
            NotSeen => match other {
                NotSeen => Ordering::Equal,
                _ => Ordering::Greater,
            },
            OverDue(days) => match other {
                NotSeen => Ordering::Less,
                OverDue(other_days) => days.cmp(other_days),
                DueIn(_) => Ordering::Greater,
            },
            DueIn(days) => match other {
                NotSeen => Ordering::Less,
                OverDue(_) => Ordering::Less,
                DueIn(other_days) => days.cmp(other_days).reverse(),
            },
        }
    }
}

impl PartialOrd for DueDays {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub struct UpcomingFriends {
    queue_by_due_days: PriorityQueue<Friend, DueDays>,
}

impl UpcomingFriends {
    pub fn new() -> Self {
        Self {
            queue_by_due_days: PriorityQueue::new(),
        }
    }

    pub fn push(&mut self, friend: Friend, today: NaiveDate) -> Result<()> {
        friend.days_until_due(today).map(|due_days| match due_days {
            DueDays::DueIn(days) if days > UP_SOON_CUTOFF_DAYS => {
                // Don't track this far into the future
            }
            due_days => {
                self.queue_by_due_days.push(friend, due_days);
            }
        })
    }

    pub fn print(&self) {
        let mut table = Table::new();
        table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
        table.set_titles(Friend::get_table_titles(true));

        for (friend, due_days) in self.queue_by_due_days.clone().into_sorted_iter() {
            let row = friend.get_table_row_with_due(due_days);
            table.add_row(row);
        }

        table.printstd();
    }
}
