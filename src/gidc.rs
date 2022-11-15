use crate::config::{Config, ToGitString};
use clap::{Parser, Subcommand};
use std::{fs, process::Command, str};
use toml_edit::{value, Document};
mod config;

#[derive(Parser, Debug)]
#[command(name = "gidc")]
#[command(about = "`gid` configuration utility")]
#[command(author, version, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Action,
}

#[derive(Subcommand, Debug)]
enum Action {
    /// Export profile settings to Git configuration
    Export {
        /// Export to global Git configuration. Local if not set.
        #[arg(short, long)]
        global: bool,

        /// Profile name. Active profile if not provided.
        name: Option<String>,
    },

    /// List all profiles
    List,

    /// Set active profile
    Set {
        /// Profile name
        name: String,
    },
}

fn main() {
    let cli = Cli::parse();
    let config_path = Config::detect().expect("could not detect config");
    let config_string = fs::read_to_string(&config_path).unwrap();
    let mut config = Config::parse(&config_string).unwrap();
    let mut config_doc = config_string.parse::<Document>().unwrap();

    match &cli.command {
        Action::Export { global, name } => {
            let mut profile = name;
            if let None = name {
                profile = &config.active;
            }

            let profile = profile
                .as_ref()
                .expect("no profile provided and no active profile");
            let profile = config
                .profiles
                .get(profile)
                .expect(format!("profile '{}' not found'", profile).as_str());

            let base_args = vec!["config", if *global { "--global" } else { "--local" }];

            for (key, val) in profile.fields.iter() {
                Command::new("git")
                    .args(&base_args)
                    .arg(key)
                    .arg(val.to_git_string())
                    .status()
                    .expect("failed to execute Git command");
            }
        }
        Action::List => {
            let active = if let Some(a) = config.active {
                a
            } else {
                "".to_string()
            };
            for profile in config.profiles.iter() {
                if profile.name == active {
                    println!("* {}", profile.name);
                } else {
                    println!("  {}", profile.name);
                }
            }
        }
        Action::Set { name } => {
            if let Some(_) = config.profiles.get(name) {
                config.active = Some(name.to_string());
            } else {
                panic!("profile name not found in configuration");
            }
            config_doc["active"] = value(name);
            fs::write(&config_path, config_doc.to_string()).unwrap();
        }
    }
}
