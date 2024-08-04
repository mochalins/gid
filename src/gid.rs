use gid::{Config, ToGitString};
use std::{env, process::Command};

fn main() {
    let config_error_message = "No configuration detected. Learn how to \
                                initialize an empty configuration file with
                                `gidc help init`.";

    let config = Config::detect().expect(config_error_message);
    let config = Config::parse_file(&config).unwrap();

    let profile = config.active.expect("no active profile");
    let profile = config
        .profiles
        .get(&profile)
        .expect(format!("profile '{}' not found", profile).as_str());

    let mut user_args: Vec<String> = env::args().collect();
    user_args.remove(0);

    let mut config_args: Vec<String> = Vec::new();

    for (key, val) in profile.fields.iter() {
        config_args.push("-c".to_string());
        config_args.push(format!("{}={}", key, val.to_git_string()));
    }

    Command::new("git")
        .args(config_args)
        .args(user_args)
        .status()
        .expect("failed to execute Git command");
}
