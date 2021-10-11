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

    if let Some(matches) = matches.subcommand_matches(ListFriends::ID) {
        ListFriends::execute(matches);
    }
    if let Some(matches) = matches.subcommand_matches(AddFriend::ID) {
        AddFriend::execute(matches);
    }
}

pub trait Command {
    const ID: &'static str;

    fn as_subcommand<'a, 'b>() -> App<'a, 'b>;

    fn execute(matches: &ArgMatches);
}

struct ListFriends {}

impl Command for ListFriends {
    const ID: &'static str = "list";

    fn as_subcommand<'a, 'b>() -> App<'a, 'b> {
        SubCommand::with_name(Self::ID)
            .about("List all of your friends")
    }

    fn execute(_: &ArgMatches) {
        println!("Listing all of your friends...");
    }
}

struct AddFriend {}

impl Command for AddFriend {
    const ID: &'static str = "add";

    fn as_subcommand<'a, 'b>() -> App<'a, 'b> {
        SubCommand::with_name(Self::ID)
            .about("Add a friend")
            .arg(Arg::with_name("friend")
                .help("Name of the friend")
                .required(true))
    }

    fn execute(matches: &ArgMatches) {
        println!("Adding a friend {}...", matches.value_of("friend").unwrap());
    }
}
