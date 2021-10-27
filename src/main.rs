#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

mod commands;
mod db;
mod models;
mod schema;

use crate::commands::*;
use db::SqliteConnection;
use diesel_migrations::embed_migrations;
use structopt::StructOpt;

embed_migrations!();

fn main() {
    let opt = FriendGrow::from_args();

    let conn = db::connect().expect("Failed to connect to database");
    embedded_migrations::run(&conn).expect("Failed to run migration");

    execute_command(opt, &conn);
}

#[derive(StructOpt, Debug)]
#[structopt(
    name = "FriendGrow",
    about = "Let your friendships grow",
    version = "0.1",
    author = "annapapitto"
)]
enum FriendGrow {
    #[structopt(name = "list", about = "List all of your friends")]
    ListFriends {},

    #[structopt(name = "show", about = "Show a friend")]
    ShowFriend { name: String },

    #[structopt(name = "add", about = "Add a friend")]
    AddFriend {
        name: String,

        #[structopt(help = "Where they live")]
        location: String,

        #[structopt(short, help = "How often to see them, in weeks")]
        freq_weeks: Option<i32>,
    },

    #[structopt(name = "remove", about = "Remove a friend")]
    RemoveFriend { name: String },

    #[structopt(name = "set-freq", about = "Set how often to see a friend")]
    SetFrequency {
        name: String,
        #[structopt(help = "How often to see them, in weeks")]
        freq_weeks: i32,
    },

    #[structopt(name = "record", about = "Record seeing a friend")]
    RecordSeen {
        name: String,

        #[structopt(help = "Date seen in YYYY-MM-DD")]
        date: String,
    },
}

fn execute_command(opt: FriendGrow, conn: &SqliteConnection) {
    match opt {
        FriendGrow::ListFriends {} => list_friends(conn),
        FriendGrow::ShowFriend { name } => show_friend(name, conn),
        FriendGrow::AddFriend {
            name,
            location,
            freq_weeks,
        } => add_friend(name, location, freq_weeks, conn),
        FriendGrow::RemoveFriend { name } => remove_friend(name, conn),
        FriendGrow::SetFrequency { name, freq_weeks } => set_frequency(name, freq_weeks, conn),
        FriendGrow::RecordSeen { name, date } => record_seen(name, date, conn),
    };
}
