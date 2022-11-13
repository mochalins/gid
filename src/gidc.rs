use clap::{Parser, Subcommand};
use std::{fs, process::Command};
use toml_edit::{value, Document};
mod config;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Action,
}

#[derive(Subcommand, Debug)]
enum Action {
    /// List all profiles
    List,

    /// Set active profile
    Set { name: String },
}

fn main() {
    let cli = Cli::parse();
    let config_path = config::Config::detect().expect("could not detect config");
    let config_string = fs::read_to_string(&config_path).unwrap();
    let mut config = config::Config::parse(&config_string).unwrap();
    let mut config_doc = config_string.parse::<Document>().unwrap();

    match &cli.command {
        Action::List => {
            let names = config.list_profile_names();
            for name in names {
                println!("{}", name);
            }
        },
        Action::Set { name } => {
            config.set_active(name).expect("invalid profile name");
            config_doc["active"] = value(name);
            fs::write(&config_path, config_doc.to_string()).unwrap();
        }
    }
}
