use clap::{App, Arg, SubCommand};

fn main() {
    println!("Hello, world! Let's be good friends.");

    let list_sub = SubCommand::with_name("list")
            .about("List all of your friends");
    let add_sub = SubCommand::with_name("add")
            .about("Add a friend")
            .arg(Arg::with_name("friend")
                .help("Name of the friend")
                .required(true));

    let app = App::new("Friend Grow")
        .version("0.1")
        .author("annapapitto")
        .about("Let your friendships grow")
        .subcommands(vec![ list_sub, add_sub ]);
    let matches = app.get_matches();

    if let Some(_) = matches.subcommand_matches("list") {
        println!("Listing all of your friends...");
    }

    if let Some(matches) = matches.subcommand_matches("add") {
        println!("Adding a friend {}...", matches.value_of("friend").unwrap());
    }
}
