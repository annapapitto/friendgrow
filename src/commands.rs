use crate::models::*;
use crate::schema::friends;
use diesel::prelude::*;

const DEFAULT_FREQ_DAYS: i32 = 100;

pub fn list_friends(conn: SqliteConnection) {
    use self::friends::dsl::*;

    // TODO move this to db.rs?
    let results = friends
        .load::<Friend>(&conn)
        .expect("Error loading friends");

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

    // TODO move this to db.rs?
    diesel::insert_into(friends::table)
        .values(&new_friend)
        .execute(&conn)
        .expect(&format!("Error adding friend {}", name));
}
