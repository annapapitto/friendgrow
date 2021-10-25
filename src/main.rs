#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
extern crate dotenv; // TODO why was this introduced? do i need it in db.rs?

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

        #[structopt(short, help = "How often to see, in days")]
        freq_days: Option<i32>,
    },

    #[structopt(name = "remove", about = "Remove a friend")]
    RemoveFriend { name: String },

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
        FriendGrow::AddFriend { name, freq_days } => add_friend(name, freq_days, conn),
        FriendGrow::RemoveFriend { name } => remove_friend(name, conn),
        FriendGrow::RecordSeen { name, date } => record_seen(name, date, conn),
    };
}
