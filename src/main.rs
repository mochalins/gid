use std::{
    collections::HashMap,
    env,
    fs,
    process::{
        Command,
    },
};
use serde::{
    Deserialize,
};
use toml::{
    Value,
    from_str
};

#[derive(Deserialize, Debug)]
struct Config {
    default: String,
    profile: HashMap<String, Profile>,
}

#[derive(Deserialize, Debug)]
struct Profile {
    user: Option<ProfileUser>,
    commit: Option<ProfileCommit>,
    tag: Option<ProfileTag>,
    pull: Option<ProfilePull>,
    sshkey: Option<String>,
}

#[derive(Deserialize, Debug)]
struct ProfileUser {
    name: Option<String>,
    email: Option<String>,
    signingkey: Option<String>,
}

#[derive(Deserialize, Debug)]
struct ProfileCommit {
    gpgsign: Option<bool>,
}

#[derive(Deserialize, Debug)]
struct ProfileTag {
    gpgsign: Option<bool>,
}

#[derive(Deserialize, Debug)]
struct ProfilePull {
    rebase: Option<bool>,
}

fn main() {
    let mut config_path: Option<String> = None;

    // Check environment variable for config path
    let env_config_path = env::var("GID_CONFIG");
    if env_config_path.is_ok() {
        config_path = Some(env_config_path.unwrap());
    }

    // Check local working directory for config file
    if config_path.is_none() {
        let local_config_path = env::current_dir();
        if local_config_path.is_ok() {
            let mut local_config_path = local_config_path.unwrap();
            local_config_path.push("gid.toml");
            if local_config_path.try_exists().is_ok() {
                if local_config_path.try_exists().unwrap() {
                    config_path = Some(
                        String::from(local_config_path.to_str().unwrap())
                    );
                }
            }
        }
    }

    // Check config directory for config file
    if config_path.is_none() {
    }

    let config: Config = toml::from_str(
        &fs::read_to_string(
            config_path.expect("could not detect configuration file")
        ).expect("could not load configuration file")
    ).expect("could not parse configuration file");

    let profile: &Profile = &config.profile[&config.default];

    let mut user_args: Vec<String> = env::args().collect();
    user_args.remove(0);

    let mut config_args: Vec<String> = Vec::new();

    match &profile.user {
        Some(u) => {
            match &u.name {
                Some(n) => {
                    config_args.push("-c".to_string());
                    config_args.push(format!("user.name={}", n));
                }, None => {},
            }
            match &u.email {
                Some(e) => {
                    config_args.push("-c".to_string());
                    config_args.push(format!("user.email={}", e));
                }, None => {}
            }
            match &u.signingkey {
                Some(s) => {
                    config_args.push("-c".to_string());
                    config_args.push(format!("user.signingkey={}", s));
                }, None => {}
            }
        }, None => {}
    }

    Command::new("git")
        .args(config_args)
        .args(user_args)
        .status().expect("failed to execute git command");
}
