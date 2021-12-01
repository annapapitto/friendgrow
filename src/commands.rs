use crate::db::{self, SqliteConnection};
use crate::models::*;
use crate::upcoming::UpcomingFriends;
use crate::{dates, ListOrderBy};
use anyhow::{Context, Result};
use prettytable::{format, Table};

const DEFAULT_FREQ_WEEKS: i32 = 10;

pub fn list_friends(order_by: ListOrderBy, conn: &SqliteConnection) -> Result<()> {
    let all_friends =
        db::load_all_friends_ordered(order_by.into(), conn).context("Failed to load friends")?;
    if all_friends.is_empty() {
        return Err(anyhow::anyhow!(
            "No friends yet. Add some with the `add` command."
        ));
    }

    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
    table.set_titles(Friend::get_table_titles(false));

    for friend in all_friends {
        table.add_row(friend.get_table_row());
    }

    table.printstd();
    Ok(())
}

pub fn show_friend(name: String, conn: &SqliteConnection) -> Result<()> {
    let friend = db::load_friend(&name, conn).context("Failed to load friend")?;
    println!("{}", friend);
    Ok(())
}

pub fn add_friend(
    name: String,
    location: String,
    freq_weeks: Option<i32>,
    conn: &SqliteConnection,
) -> Result<()> {
    let new_friend = NewFriend {
        name: name.clone(),
        location: location.clone(),
        freq_weeks: freq_weeks.unwrap_or(DEFAULT_FREQ_WEEKS),
    };

    db::insert_friend(new_friend, conn).context("Failed to add friend")?;
    show_friend(name, conn)
}

pub fn remove_friend(name: String, conn: &SqliteConnection) -> Result<()> {
    show_friend(name.clone(), conn)?;
    db::delete_friend(&name, conn).context("Failed to remove friend")?;
    Ok(())
}

pub fn set_name(curr_name: String, new_name: String, conn: &SqliteConnection) -> Result<()> {
    db::update_name(&curr_name, &new_name, conn).context("Failed to set name")?;
    show_friend(new_name, conn)
}

pub fn set_location(name: String, location: String, conn: &SqliteConnection) -> Result<()> {
    db::update_location(&name, location, conn).context("Failed to set location")?;
    show_friend(name, conn)
}

pub fn set_frequency(name: String, freq_weeks: i32, conn: &SqliteConnection) -> Result<()> {
    dates::check_frequency(freq_weeks)?;

    db::update_freq_weeks(&name, freq_weeks, conn).context("Failed to set frequency")?;
    show_friend(name, conn)
}

pub fn record_seen(name: String, date: String, conn: &SqliteConnection) -> Result<()> {
    let new_date = dates::parse_date(&date)?;

    let friend = db::load_friend(&name, conn).context("Failed to load friend")?;
    dates::check_new_seen(new_date, friend.last_seen)?;

    db::update_last_seen(&name, new_date.to_string(), conn).context("Failed to record seen")?;
    show_friend(name, conn)
}

pub fn list_upcoming(conn: &SqliteConnection) -> Result<()> {
    let results = db::load_all_friends(conn).context("Failed to load friends")?;
    let today = dates::local_today();
    let mut upcoming_friends = UpcomingFriends::new();

    for friend in results {
        upcoming_friends.push(friend.clone(), today)?;
    }

    upcoming_friends.print();
    Ok(())
}
