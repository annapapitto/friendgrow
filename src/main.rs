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

    let conn = db::connect();
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

    #[structopt(name = "add", about = "Add a friend")]
    AddFriend { name: String },

    #[structopt(name = "record", about = "Record seeing a friend as YYYY-MM-DD")]
    RecordSeen { name: String, date: String },
}

fn execute_command(opt: FriendGrow, conn: &SqliteConnection) {
    match opt {
        FriendGrow::ListFriends {} => list_friends(conn),
        FriendGrow::AddFriend { name } => add_friend(name, conn),
        FriendGrow::RecordSeen { name, date } => record_seen(name, date, conn),
    };
}
