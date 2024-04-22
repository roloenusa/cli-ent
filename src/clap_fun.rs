use clap::{arg, Arg, ArgAction, Command};

pub fn clap_fun() {
    let cmd = Command::new("pacman")
        .about("package manager utility")
        .version("5.2.1")
        // .subcommand_required(true)
        .arg_required_else_help(true)
        // Query subcommand
        //
        // Only a few of its arguments are implemented below.
        .subcommand(
            Command::new("query")
                .short_flag('Q')
                .long_flag("query")
                .about("Query the package database.")
                .arg(
                    Arg::new("search")
                        .short('s')
                        .long("search")
                        .help("search locally installed packages for matching strings")
                        .conflicts_with("info")
                        .action(ArgAction::Set)
                        .num_args(1..),
                )
                .arg(
                    Arg::new("info")
                        .long("info")
                        .short('i')
                        .conflicts_with("search")
                        .help("view package information")
                        .action(ArgAction::Set)
                        .num_args(1..),
                ),
        )
        // Sync subcommand
        //
        // Only a few of its arguments are implemented below.
        .subcommand(
            Command::new("sync")
                .short_flag('S')
                .long_flag("sync")
                .about("Synchronize packages.")
                .arg(
                    Arg::new("search")
                        .short('s')
                        .long("search")
                        .conflicts_with("info")
                        .action(ArgAction::Set)
                        .num_args(1..)
                        .help("search remote repositories for matching strings"),
                )
                .arg(
                    Arg::new("info")
                        .long("info")
                        .conflicts_with("search")
                        .short('i')
                        .action(ArgAction::SetTrue)
                        .help("view package information"),
                )
                .arg(
                    Arg::new("package")
                        .help("packages")
                        .required_unless_present("search")
                        .action(ArgAction::Set)
                        .num_args(1..),
                ),
        )
        .arg(
            Arg::new("Markdown")
                .short('m')
                .long("markdown")
                .help("This is the markdown"),
        )
        .arg(
            Arg::new("Test")
                .short('t')
                .long("test")
                .action(ArgAction::Set)
                .value_name("VALUE_NAME")
                .value_names(vec!["Test1", "Test2"])
                .value_delimiter(',')
                .help("This is another test"),
        )
        .arg(
            Arg::new("Barb")
                .short('r')
                .long("barb")
                .value_name("BARB")
                .help("This is another test"),
        )
        .arg(
            Arg::new("other")
                .long("o")
                .value_name("OTHER")
                .help("This is another test"),
        )
        .arg(
            Arg::new("")
                .short('p')
                .value_name("testtest")
                .action(ArgAction::Set)
                .help("This is another test"),
        )
        .arg(
            Arg::new("queue")
                .short('q')
                .action(ArgAction::SetTrue)
                .help("This is another test"),
        )
        // .arg(
        //     Arg::new("positional0")
        //     .action(ArgAction::Append)
        //     .value_names(["fast", "slow"])
        //     .default_value("fast")
        //     .help("This is the help for somethig or th eother")
        // )
        // .arg(
        //     Arg::new("positional11")
        //     .help("This is the help for somethig or th eother")
        // )
        // .arg(
        //     Arg::new("positional222")
        //     .last(true)
        //     .help("This is the help for somethig or th eother")
        // )
        .arg(arg!(-f --file <FILE_NAME> "FILE NAME is a required value"))
        .arg(arg!(-d --dir [DIR_NAME] "DIR NAME is an optional value"))
        .arg(arg!(foldername: -o --folder [FOLDER_NAME] "FOLDER NAME is an optional value"))
        .arg(arg!(-a --adventure ... "Can be used multple times"))
        .arg(arg!(positional: [POSITIONAL] "This is positional"));

    let matches = cmd.clone().get_matches();

    match matches.subcommand() {
        Some(("sync", sync_matches)) => {
            if sync_matches.contains_id("search") {
                let packages: Vec<_> = sync_matches
                    .get_many::<String>("search")
                    .expect("contains_id")
                    .map(|s| s.as_str())
                    .collect();
                let values = packages.join(", ");
                println!("Searching for {values}...");
                return;
            }

            let packages: Vec<_> = sync_matches
                .get_many::<String>("package")
                .expect("is present")
                .map(|s| s.as_str())
                .collect();
            let values = packages.join(", ");

            if sync_matches.get_flag("info") {
            } else {
                println!("Retrieving info for {values}...");
                println!("Installing {values}...");
            }
        }
        Some(("query", query_matches)) => {
            if let Some(packages) = query_matches.get_many::<String>("info") {
                let comma_sep = packages.map(|s| s.as_str()).collect::<Vec<_>>().join(", ");
                println!("Retrieving info for {comma_sep}...");
            } else if let Some(queries) = query_matches.get_many::<String>("search") {
                let comma_sep = queries.map(|s| s.as_str()).collect::<Vec<_>>().join(", ");
                println!("Searching Locally for {comma_sep}...");
            } else {
                println!("Displaying all locally installed packages...");
            }
        }
        // _ => unreachable!(), // If all subcommands are defined above, anything else is unreachable
        _ => (), // If all subcommands are defined above, anything else is unreachable
    }

    match matches.get_one::<String>("Markdown") {
        Some(_) => clap_show::help_command(&cmd),
        _ => "".to_string(),
    };

    // match  matches.get_one::<String>("file") {
    //     v => println!("----- 1: {:#?}", v)
    // };

    // match  matches.get_one::<bool>("queue") {
    //     v => println!("----- 2: {:#?}", v)
    // };

    // match  matches.get_one::<String>("dir") {
    //     v => println!("----- 3: {:#?}", v)
    // };

    // match  matches.get_one::<String>("foldername") {
    //     v => println!("----- 4: {:#?}", v)
    // };

    // match  matches.get_one::<u8>("adventure") {
    //     v => println!("----- 5: {:#?}", v)
    // };

    // match  matches.get_one::<String>("other") {
    //     v => println!("----- 6: {:#?}", v)
    // };
}
