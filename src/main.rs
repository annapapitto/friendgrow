use clap::{App, Arg, ArgMatches, SubCommand};

fn main() {
    println!("Hello, world! Let's be good friends.");

    let subcommands = vec![
        ListFriends::as_subcommand(),
        AddFriend::as_subcommand()
    ];

    let app = App::new("Friend Grow")
        .version("0.1")
        .author("annapapitto")
        .about("Let your friendships grow")
        .subcommands(subcommands);
    let matches = app.get_matches();

    if let Some(matches) = matches.subcommand_matches("list") {
        ListFriends::execute(matches);
    }
    if let Some(matches) = matches.subcommand_matches("add") {
        AddFriend::execute(matches);
    }
}

pub trait Command {
    fn as_subcommand<'a, 'b>() -> App<'a, 'b>;

    fn execute(matches: &ArgMatches);
}

struct ListFriends {}

impl Command for ListFriends {
    fn as_subcommand<'a, 'b>() -> App<'a, 'b> {
        SubCommand::with_name("list")
            .about("List all of your friends")
    }

    fn execute(_: &ArgMatches) {
        println!("Listing all of your friends...");
    }
}

struct AddFriend {}

impl Command for AddFriend {
    fn as_subcommand<'a, 'b>() -> App<'a, 'b> {
        SubCommand::with_name("add")
            .about("Add a friend")
            .arg(Arg::with_name("friend")
                .help("Name of the friend")
                .required(true))
    }

    fn execute(matches: &ArgMatches) {
        println!("Adding a friend {}...", matches.value_of("friend").unwrap());
    }
}
