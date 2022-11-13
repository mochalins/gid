use std::{env, process::Command};
mod config;

fn main() {
    let config = config::Config::detect().expect("could not detect config");
    let config = config::Config::parse_file(&config).unwrap();

    let profile = config.get_active_profile().expect("no active profile");

    let mut user_args: Vec<String> = env::args().collect();
    user_args.remove(0);

    let mut config_args: Vec<String> = Vec::new();

    if let Some(n) = profile.user_name() {
        config_args.push("-c".to_string());
        config_args.push(format!("user.name={}", n));
    }
    if let Some(e) = profile.user_email() {
        config_args.push("-c".to_string());
        config_args.push(format!("user.email={}", e));
    }
    if let Some(s) = profile.user_signingkey() {
        config_args.push("-c".to_string());
        config_args.push(format!("user.signingkey={}", s));
    }
    if let Some(g) = profile.commit_gpgsign() {
        config_args.push("-c".to_string());
        config_args.push(format!("commit.gpgsign={}", g));
    }
    if let Some(g) = profile.tag_gpgsign() {
        config_args.push("-c".to_string());
        config_args.push(format!("tag.gpgsign={}", g));
    }
    if let Some(r) = profile.pull_rebase() {
        config_args.push("-c".to_string());
        config_args.push(format!("pull.rebase={}", r));
    }
    if let Some(s) = profile.sshkey() {
        config_args.push("-c".to_string());
        config_args.push(format!("core.sshCommand=ssh -i \"{}\"", s));
    }

    Command::new("git")
        .args(config_args)
        .args(user_args)
        .status()
        .expect("failed to execute Git command");
}
