// TODO what are these for?
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
//#[macro_use] // TODO do you need one of these per 'extern crate'?
//extern crate dotenv; // TODO why was this introduced? do i need it in db.rs?

// TODO why do I need models and schema if I don't use them? do the examples use them here?
pub mod commands;
pub mod db;
pub mod models;
pub mod schema;

use crate::commands::*;
use diesel::prelude::*;
use diesel_migrations::embed_migrations;
use structopt::StructOpt;

embed_migrations!();

fn main() {
    let opt = FriendGrow::from_args();

    let conn = db::connect();
    embedded_migrations::run(&conn).expect("Failed to run migration");

    execute_command(opt, conn);
}

#[derive(StructOpt, Debug)]
#[structopt(
    name = "FriendGrow",
    about = "Let your friendships grow",
    version = "0.1",
    author = "annapapitto"
)]
enum FriendGrow {
    #[structopt(name = "list", help = "List all of your friends")]
    ListFriends {},

    #[structopt(name = "add", help = "Add a friend")]
    AddFriend { friend: String },
}

fn execute_command(opt: FriendGrow, conn: SqliteConnection) {
    match opt {
        FriendGrow::ListFriends {} => list_friends(conn),
        FriendGrow::AddFriend { friend } => add_friend(friend, conn),
    };
}
