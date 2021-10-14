use rusqlite::{params, Connection, Result};
use structopt::StructOpt;

const DEFAULT_FREQ_DAYS: i32 = 100;

fn main() -> Result<()> {
    let opt = FriendGrow::from_args();

    let conn = Connection::open("friends.db")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS friend (
            id integer primary key,
            name text not null unique,
            freq_days integer,
            last_seen text
        )",
        [],
    )?;

    execute_subcommand(opt, conn)?;
    Ok(())
}

#[derive(StructOpt)]
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

#[derive(Debug)]
struct Friend {
    id: i32,
    name: String,
    freq_days: i32,
    last_seen: Option<String>,
}

fn execute_subcommand(opt: FriendGrow, conn: Connection) -> Result<()> {
    match opt {
        FriendGrow::ListFriends {} => {
            return list_friends(conn);
        }
        FriendGrow::AddFriend { friend } => {
            return add_friend(friend, conn);
        }
    }
}

fn list_friends(conn: Connection) -> Result<()> {
    let mut stmt = conn.prepare("SELECT id, name, freq_days, last_seen FROM friend")?;
    let rows = stmt.query_map([], |row| {
        Ok(Friend {
            id: row.get(0)?,
            name: row.get(1)?,
            freq_days: row.get(2)?,
            last_seen: row.get(3)?,
        })
    })?;

    for friend_result in rows {
        println!("{:?}", friend_result);
    }

    Ok(())
}

fn add_friend(friend: String, conn: Connection) -> Result<()> {
    conn.execute(
        "INSERT INTO friend (name, freq_days) values (?1, ?2)",
        params![friend, DEFAULT_FREQ_DAYS],
    )?;

    Ok(())
}
