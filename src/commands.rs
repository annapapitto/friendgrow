use crate::db::*;
use crate::models::*;

const DEFAULT_FREQ_DAYS: i32 = 100;

pub fn list_friends(conn: SqliteConnection) {
    let results = load_all_friends(conn).expect("Error getting friends");

    for friend in results {
        let seen_str: String = friend.last_seen.map_or("Never seen".to_string(), |last| {
            format!("Last seen {} days ago", last)
        });
        println!(
            "{}\tEvery {} days\t{}",
            friend.name, friend.freq_days, seen_str
        );
    }
}

pub fn add_friend(name: String, conn: SqliteConnection) {
    let new_friend = NewFriend {
        name: name.clone(),
        freq_days: DEFAULT_FREQ_DAYS,
    };

    insert_friend(new_friend, conn).expect(&format!("Error adding friend {}", name));
}
