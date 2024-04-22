#[allow(dead_code)]
#[allow(unused_imports)]
use clap::{command, ArgMatches, CommandFactory, Parser};

#[allow(unused_imports)]
use clap::Subcommand;
// use std::error::Error;

// use std::path::PathBuf;
use clap::{arg, /*value_parser, ArgAction,*/ Command};

mod git;
mod octo;
mod ghlib;

trait Exec {
    fn execute();
}

// struct Subcommand {

// }

// impl Exec for Subcommand {
//     fn execute() {
//         println!("---- this is subcommand");
//     }
// }

/// TESTING ONE TWO THREE
#[derive(Parser)]
#[command(name = "complex-app")]
pub struct Cli {
    /// This is a simple test command
    name: Option<String>,
}

// #[tokio::main]
fn main() {
    // let matches = Command::new("MyApp")
    //     .arg(arg!([name] "Optional name to operate on"))
    //     .arg(
    //         arg!(
    //             -c --config <FILE> "Sets a custom config file"
    //         )
    //         // We don't have syntax yet for optional options, so manually calling `required`
    //         .required(false)
    //         .value_parser(value_parser!(PathBuf)),
    //     )
    //     .arg(arg!(
    //         -d --debug ... "Turn debugging information on"
    //     ))
    //     .subcommand(
    //         Command::new("test")
    //             .about("does testing things")
    //             .arg(arg!(-l --list "lists test values").action(ArgAction::SetTrue)),
    //     )
    //     .get_matches();

    // // You can check the value provided by positional arguments, or option arguments
    // if let Some(name) = matches.get_one::<String>("name") {
    //     println!("Value for name: {name}");
    // }

    // if let Some(config_path) = matches.get_one::<PathBuf>("config") {
    //     println!("Value for config: {}", config_path.display());
    // }

    // // You can see how many times a particular flag or argument occurred
    // // Note, only flags can have multiple occurrences
    // match matches
    //     .get_one::<u8>("debug")
    //     .expect("Count's are defaulted")
    // {
    //     0 => println!("Debug mode is off"),
    //     1 => println!("Debug mode is kind of on"),
    //     2 => println!("Debug mode is on"),
    //     _ => println!("Don't be crazy"),
    // }

    // // You can check for the existence of subcommands, and if found use their
    // // matches just as you would the top level cmd
    // let m = matches.subcommand();
    // let (name, _other) = m.unwrap();
    // println!("--- test {:#?}", name);

    // if let Some(matches) = matches.subcommand_matches("test") {
    //     // "$ myapp test" was run
    //     if matches.get_flag("list") {
    //         // "$ myapp test -l" was run
    //         println!("Printing testing lists...");
    //     } else {
    //         println!("Not printing testing lists...");
    //     }
    // }

    let cmd = Command::new("MyApp")
        .arg(arg!(-g --git "Do stuff on git"))
        // .arg(arg!(-o --octo "Do stuff on octo"))
        .arg(arg!(-l --ghlib "Do stuff on octo"))
        .arg(arg!(-m --md "Markdown parser"))
        .arg(arg!(-b --masdfasdfasdfsdf [abcfhijklmnph] "This is a short THIS IS A VERY LONG COMMAND IS VERY VERY VEY LONG"));

    let matches = cmd.clone().get_matches();

    // You can check the value provided by positional arguments, or option arguments
    match  matches.get_one::<bool>("git") {
        Some(true) => {
            git::git_binding();
        },
        _ => ()
    };

    // match  matches.get_one::<bool>("octo") {
    //     Some(true) => {
    //         octo::octo_binding().await
    //     },
    //     _ => ()
    // };

    match  matches.get_one::<bool>("ghlib") {
        Some(true) => {
            ghlib::ghlib_binding()
        },
        _ => ()
    };

    match  matches.get_one::<bool>("md") {
        Some(true) => {
            println!("{}", clap_show::help_markdown_command(&cmd));
        },
        _ => ()
    };

    // Continued program logic goes here...
}
