mod maigacha;
use crate::maigacha::{Pull, PullList, PullType};

use std::path::{Path, PathBuf};
use structopt::StructOpt;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

const MAIGACHA_FILE: &str = "maigacha.json";
const RESET: &str = "\x1b[0m";

const GREEN: &str = "\x1b[32m";
const YELLOW: &str = "\x1b[33m";

fn main() -> Result<()> {
    let args = Cli::from_args();
    let path = if let Some(path) = args.file {
        path
    } else {
        get_default_file()?
    };
    let mut pull_list = get_maigacha_list(&path);
    match args.command {
        Command::Add {
            name,
            pull_type,
            chance,
        } => {
            if chance > 0_f64 {
                pull_list.insert(Pull::new(name, pull_type, chance));
            } else {
                println!("chance can't be 0 or less.");
            }
        }
        Command::Remove { name } => {
            if pull_list.remove(&name).is_some() {
                println!(r#""{name}", has been removed."#);
            } else {
                println!(r#""{name}", not in list."#);
            }
        }
        Command::Pull => pull_list.pull().map_or_else(
            || {
                println!("Nothing to pull.");
            },
            |pull| {
                let color = match pull.pull_type {
                    PullType::Common => GREEN,
                    PullType::Rare => YELLOW,
                };
                println!(
                    "Pulled a {color}{:#?}{RESET}\n{:#?} : {:#?}",
                    pull.pull_type, pull.name, pull.chance
                );
            },
        ),
        Command::List => {
            pull_list.print_list();
        }
        Command::History => {
            pull_list.pull_history.print();
        }
    }
    pull_list.save_to_json(path.to_str().unwrap())?;
    Ok(())
}

#[derive(Debug, StructOpt)]
enum Command {
    /// Add an item to the list.
    ///
    /// Add format is <name> <common/rare> <chance>
    #[structopt(alias = "a")]
    Add {
        name: String,
        pull_type: PullType,
        chance: f64,
    },
    /// Remove an item from the list.
    #[structopt(alias = "r")]
    Remove { name: String },
    /// Pulls an item from the list.
    #[structopt(alias = "p")]
    Pull,
    /// Shows the list.
    #[structopt(alias = "l")]
    List,
    /// Shows the history.
    #[structopt(alias = "h")]
    History,
}

#[derive(Debug, StructOpt)]
#[structopt(name = "Maigacha")]
struct Cli {
    #[structopt(subcommand)]
    command: Command,
    /// File to use for the commands.
    /// Creates a file if file doesnt exist.
    /// Defaults to ~/.config/maigach/maigacha.json
    /// or %appdata%\maigacha\maigacha.json
    #[structopt(short = "f", long = "file")]
    file: Option<PathBuf>,
}

fn get_maigacha_list(path: &Path) -> PullList {
    if path.exists() {
        return PullList::load_from_json_file(path.to_str().unwrap()).unwrap_or(PullList::new());
    }
    PullList::new()
}

fn get_default_file() -> Result<PathBuf> {
    let file_name = MAIGACHA_FILE;
    let mut path = if let Some(mut path) = dirs::config_dir() {
        path.push("maigacha");
        path
    } else {
        dirs::home_dir()
            .ok_or("Could not determine home directory")?
            .join(".maigacha")
    };

    if !path.exists() {
        std::fs::create_dir_all(&path)?;
    }

    path.push(file_name);
    Ok(path)
}
