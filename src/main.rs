use rusqlite::{params, Connection, Result};
use structopt::StructOpt;

fn main() -> Result<()> {
    println!("Hello, world! Let's be good friends.");

    let opt = Friendgrow::from_args();

    // TODO make this an arg
    let conn = Connection::open("~/friends/friends.db")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS friend (
            id integer primary key,
            name text not null unique,
            freq integer,
            last_seen text,
        )",
        [],
    )?;

    execute_subcommand(opt, conn)?;
    Ok(())
}

#[derive(StructOpt)]
#[structopt(
    name = "Friend Grow",
    about = "Let your friendships grow",
    version = "0.1",
    author = "annapapitto"
)]
enum Friendgrow {
    #[structopt(name = "list", help = "List all of your friends")]
    ListFriends {},

    #[structopt(name = "add", help = "Add a friend")]
    AddFriend { friend: String },
}

#[derive(Debug)]
struct Friend {
    id: i32,
    name: String,
    freq: u8,             // TODO try with u8
    last_hangout: String, // TODO should this be Option if no 'not null'?
}

fn execute_subcommand(opt: Friendgrow, conn: Connection) -> Result<()> {
    match opt {
        Friendgrow::ListFriends {} => {
            return list_friends(conn);
        }
        Friendgrow::AddFriend { friend } => {
            return add_friend(friend, conn);
        }
    }
}

fn list_friends(conn: Connection) -> Result<()> {
    println!("Listing all of your friends...");
    let names = conn.execute("SELECT name FROM friend", [])?;
    println!("names: {:?}", names);
    Ok(())
}

fn add_friend(friend: String, conn: Connection) -> Result<()> {
    println!("Adding a friend {}...", friend);
    Ok(())
}
