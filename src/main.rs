use structopt::StructOpt;

fn main() {
    println!("Hello, world! Let's be good friends.");

    let opt = Friendgrow::from_args();
    execute_subcommand(opt);
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

fn execute_subcommand(opt: Friendgrow) {
    match opt {
        Friendgrow::ListFriends {} => {
            list_friends();
        }
        Friendgrow::AddFriend { friend } => {
            add_friend(friend);
        }
    }
}

fn list_friends() {
    println!("Listing all of your friends...");
}

fn add_friend(friend: String) {
    println!("Adding a friend {}...", friend);
}
