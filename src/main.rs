use clap::App;

fn main() {
    println!("Hello, world! Let's be good friends.");

    let subcommands = vec![
        list::as_subcommand(),
        add::as_subcommand()
    ];

    let app = App::new("Friend Grow")
        .version("0.1")
        .author("annapapitto")
        .about("Let your friendships grow")
        .subcommands(subcommands);
    let matches = app.get_matches();

    if let Some(_) = matches.subcommand_matches("list") {
        list::execute();
    }
    if let Some(matches) = matches.subcommand_matches("add") {
        add::execute(matches);
    }
}

mod list {
    use clap::{App, SubCommand};

    pub fn as_subcommand<'a, 'b>() -> App<'a, 'b> {
        SubCommand::with_name("list")
            .about("List all of your friends")
    }

    pub fn execute() {
        println!("Listing all of your friends...");
    }
}

mod add {
    use clap::{App, Arg, ArgMatches, SubCommand};

    pub fn as_subcommand<'a, 'b>() -> App<'a, 'b> {
        SubCommand::with_name("add")
            .about("Add a friend")
            .arg(Arg::with_name("friend")
                .help("Name of the friend")
                .required(true))
    }

    pub fn execute(matches: &ArgMatches) {
        println!("Adding a friend {}...", matches.value_of("friend").unwrap());
    }
}
