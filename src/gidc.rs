use clap::{Parser, Subcommand};
use std::{fs, process::Command};
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
    let config_path = config::Config::detect().expect("could not detect config");
    let config_string = fs::read_to_string(&config_path).unwrap();
    let mut config = config::Config::parse(&config_string).unwrap();
    let mut config_doc = config_string.parse::<Document>().unwrap();

    match &cli.command {
        Action::Export { global, name } => {
            let mut profile = config.get_active_profile();
            if let Some(n) = name {
                profile = config.get_profile(n);
            }
            let profile = profile.expect("profile not found");

            let base_args = vec!["config", if *global { "--global" } else { "--local" }];

            if let Some(n) = profile.user_name() {
                Command::new("git")
                    .args(&base_args)
                    .arg("user.name")
                    .arg(n)
                    .status()
                    .expect("failed to execute Git command");
            }
            if let Some(e) = profile.user_email() {
                Command::new("git")
                    .args(&base_args)
                    .arg("user.email")
                    .arg(e)
                    .status()
                    .expect("failed to execute Git command");
            }
            if let Some(s) = profile.user_signingkey() {
                Command::new("git")
                    .args(&base_args)
                    .arg("user.signingkey")
                    .arg(s)
                    .status()
                    .expect("failed to execute Git command");
            }
            if let Some(g) = profile.commit_gpgsign() {
                Command::new("git")
                    .args(&base_args)
                    .arg("commit.gpgsign")
                    .arg(g.to_string())
                    .status()
                    .expect("failed to execute Git command");
            }
            if let Some(g) = profile.tag_gpgsign() {
                Command::new("git")
                    .args(&base_args)
                    .arg("tag.gpgsign")
                    .arg(g.to_string())
                    .status()
                    .expect("failed to execute Git command");
            }
            if let Some(r) = profile.pull_rebase() {
                Command::new("git")
                    .args(&base_args)
                    .arg("pull.rebase")
                    .arg(r.to_string())
                    .status()
                    .expect("failed to execute Git command");
            }
            if let Some(s) = profile.sshkey() {
                Command::new("git")
                    .args(&base_args)
                    .arg("core.sshCommand")
                    .arg(format!("ssh -i \"{}\"", s))
                    .status()
                    .expect("failed to execute Git command");
            }
        }
        Action::List => {
            let names = config.list_profile_names();
            for name in names {
                println!("{}", name);
            }
        }
        Action::Set { name } => {
            config.set_active(name).expect("invalid profile name");
            config_doc["active"] = value(name);
            fs::write(&config_path, config_doc.to_string()).unwrap();
        }
    }
}
