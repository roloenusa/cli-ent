use clap::{Arg, Args, CommandFactory, Parser, Subcommand};
use octocrab::markdown;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Positional {
    ///
    /// Positional required argument
    /// Long description of the positional argument
    pos_required: String,

    /// Positional optional argument
    ///
    /// Long description of the positional argument
    pos_optional: Option<String>,

}

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands
}

#[derive(Subcommand)]
enum Commands {
    /// Adds files to myapp
    Add(AddArgs),

    /// Positional short help
    ///
    /// Positional arguments long help
    Positional(Positional),

    /// flag short help
    ///
    /// flag arguments long help
    Flags(FlagArgs),

    /// markdown short help
    ///
    /// markdown arguments long help
    Markdown(MarkdownArgs),
}

#[derive(Args)]
struct AddArgs {
    optional: Option<String>,

    multiple: Vec<String>,
}


#[derive(Args)]
struct FlagArgs {
    /// Flag short help
    ///
    /// Flag long help
    #[arg(short='f', long="flag")]
    flag: String,
}


#[derive(Args)]
struct MarkdownArgs {}

pub fn clap_derive() {
    let cli = Cli::parse();
    let command = Cli::command();

    match cli.command {
       _ => clap_show::help_command(&command)
    };
}
