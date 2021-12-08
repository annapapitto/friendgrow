use crate::models::*;
use anyhow::Result;
use chrono::NaiveDate;
use prettytable::{format, Table};
use priority_queue::PriorityQueue;
use std::cmp::Ordering;

const UP_SOON_CUTOFF_DAYS: u16 = 10;

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum DueDays {
    NotSeen,
    OverDue(u16),
    DueIn(u16),
}

impl DueDays {
    pub fn display_some(&self) -> Option<String> {
        match self {
            DueDays::DueIn(d) => Some(format!("in {} days", d)),
            DueDays::OverDue(d) => Some(format!("{} days ago", d)),
            DueDays::NotSeen => None,
        }
    }
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

#[cfg(test)]
mod test {
    use priority_queue::PriorityQueue;

    use super::DueDays;

    #[test]
    fn test_queue_order() {
        let mut queue_by_due_days: PriorityQueue<&str, DueDays> = PriorityQueue::new();

        queue_by_due_days.push("first", DueDays::NotSeen);
        queue_by_due_days.push("eighth", DueDays::DueIn(16));
        queue_by_due_days.push("ninth", DueDays::DueIn(16));
        queue_by_due_days.push("fourth", DueDays::OverDue(5));
        queue_by_due_days.push("second", DueDays::NotSeen);
        queue_by_due_days.push("seventh", DueDays::DueIn(8));
        queue_by_due_days.push("fifth", DueDays::OverDue(0));
        queue_by_due_days.push("third", DueDays::OverDue(10));
        queue_by_due_days.push("sixth", DueDays::DueIn(0));

        assert_eq!(queue_by_due_days.pop().unwrap().1, DueDays::NotSeen);
        assert_eq!(queue_by_due_days.pop().unwrap().1, DueDays::NotSeen);

        assert_eq!(
            queue_by_due_days.pop(),
            Some(("third", DueDays::OverDue(10)))
        );
        assert_eq!(
            queue_by_due_days.pop(),
            Some(("fourth", DueDays::OverDue(5)))
        );
        assert_eq!(
            queue_by_due_days.pop(),
            Some(("fifth", DueDays::OverDue(0)))
        );
        assert_eq!(queue_by_due_days.pop(), Some(("sixth", DueDays::DueIn(0))));
        assert_eq!(
            queue_by_due_days.pop(),
            Some(("seventh", DueDays::DueIn(8)))
        );

        assert_eq!(queue_by_due_days.pop().unwrap().1, DueDays::DueIn(16));
        assert_eq!(queue_by_due_days.pop().unwrap().1, DueDays::DueIn(16));

        assert_eq!(queue_by_due_days.pop(), None);
    }

    #[test]
    fn test_display_due_on() {
        assert_eq!(DueDays::NotSeen.display_some(), None);
        assert_eq!(
            DueDays::OverDue(0).display_some(),
            Some("0 days ago".to_owned())
        );
        assert_eq!(
            DueDays::OverDue(5).display_some(),
            Some("5 days ago".to_owned())
        );
        assert_eq!(
            DueDays::DueIn(0).display_some(),
            Some("in 0 days".to_owned())
        );
        assert_eq!(
            DueDays::DueIn(24).display_some(),
            Some("in 24 days".to_owned())
        );
    }
}
