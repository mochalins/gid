use crate::config::{Config, FromGitStr, Profile, ToGitString, Value};
use clap::{Parser, Subcommand};
use std::{
    collections::BTreeMap,
    fs,
    io::{stdin, stdout, Write},
    process::Command,
    str,
};
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
    /// Display profile settings
    Display {
        /// Profile name. Active profile if not provided.
        name: Option<String>,
    },

    /// Export profile settings to Git configuration
    Export {
        /// Export to global Git configuration. Local if not set.
        #[arg(short, long)]
        global: bool,

        /// Profile name. Active profile if not provided.
        name: Option<String>,
    },

    /// Import Git configuration to profile
    Import {
        /// Import global Git configuration. Local if not set.
        #[arg(short, long)]
        global: bool,

        /// New or existing profile name. Active profile if not provided.
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
    let mut config = config_string.parse::<Config>().unwrap();
    let mut config_doc = config_string.parse::<Document>().unwrap();

    match &cli.command {
        Action::Display { name } => {
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

            println!("{}", profile.to_string());
        }
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
        Action::Import { global, name } => {
            let mut profile = name;
            if let None = name {
                profile = &config.active;
            }
            let profile = profile
                .as_ref()
                .expect("no profile provided and no active profile");

            if let Some(_) = config.profiles.get(profile) {
                let mut user_confirmation = String::new();
                while user_confirmation != "y"
                    && user_confirmation != "n"
                    && user_confirmation != "yes"
                    && user_confirmation != "no"
                {
                    println!(
                        "Profile {} already exists; overwrite while importing?",
                        &profile
                    );
                    print!("[Y]es, [N]o: ");
                    let _ = stdout().flush();
                    stdin()
                        .read_line(&mut user_confirmation)
                        .expect("failed to parse user input");
                    user_confirmation = user_confirmation.trim().to_lowercase();
                }

                if user_confirmation != "y" && user_confirmation != "yes" {
                    return;
                }

                config.profiles.remove(profile);
            }

            let mut new_profile: Profile = Profile {
                name: profile.to_string(),
                fields: BTreeMap::new(),
            };

            let config_string: String = str::from_utf8(
                &Command::new("git")
                    .arg("config")
                    .arg(if *global { "--global" } else { "--local" })
                    .arg("--list")
                    .output()
                    .expect("failed to execute Git command")
                    .stdout,
            )
            .expect("could not parse Git config output into valid UTF-8")
            .to_string();

            for line in config_string.lines() {
                let (key, value) = line.split_once('=').unwrap();
                if let Ok(v) = Value::from_git_str(value) {
                    new_profile.fields.insert(key.to_string(), v);
                }
            }

            config.profiles.insert(new_profile);
            fs::write(&config_path, config.to_string()).unwrap();
            println!(
                "{} configuration imported to {}",
                if *global { "Global" } else { "Local" },
                profile
            );
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
