use crate::models::*;
use chrono::NaiveDate;
use core::panic;
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

    pub fn push(&mut self, friend: Friend, today: NaiveDate) {
        match friend.days_until_due(today) {
            Some(days_until_due) => {
                self.push_seen(friend, days_until_due);
            }
            None => {
                self.push_never_seen(friend);
            }
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