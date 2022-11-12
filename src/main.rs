use std::{
    collections::HashMap,
    env,
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
    /*
    let mut user_args: Vec<String> = env::args().collect();
    user_args.remove(0);

    let mut config_args: Vec<String> = Vec::new();
    let mut name_arg = String::from("user.name=");
    name_arg.push_str("sushigiri");
    name_arg.push_str("");
    config_args.push("-c".to_string());
    config_args.push(name_arg);

    Command::new("git")
        .args(config_args)
        .args(user_args)
        .status().expect("failed to execute git command");
    */
    let toml_string = r#"
default = "sushigiri"

[profile.sushigiri]
user.name = "a0"
user.email = "b0"

[profile.senkaru]
user.name = "a0"
user.email = "b0"
"#;


    let config: Config = toml::from_str(&toml_string).unwrap();
    println!("{:?}", config);
    println!("{:?}", config.profile[&config.default].user.as_ref().unwrap().name);
}
