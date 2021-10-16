use diesel::prelude::*;

const DEFAULT_FREQ_DAYS: i32 = 100;

pub fn list_friends(conn: SqliteConnection) {
    println!("Listing friends");

    //    let mut stmt = conn.prepare("SELECT id, name, freq_days, last_seen FROM friends")?;
    //let rows = stmt.query_map([], |row| {
    //Ok(Friend {
    //id: row.get(0)?,
    //name: row.get(1)?,
    //freq_days: row.get(2)?,
    //last_seen: row.get(3)?,
    //})
    //})?;

    //for friend_result in rows {
    //println!("{:?}", friend_result);
    //}
}

pub fn add_friend(friend: String, conn: SqliteConnection) {
    println!("Adding friend {}", friend);

    //    conn.execute(
    //"INSERT INTO friends (name, freq_days) values (?1, ?2)",
    //params![friend, DEFAULT_FREQ_DAYS],
    //)?;
}
