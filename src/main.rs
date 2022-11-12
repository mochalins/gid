use std::{env, fs, process::Command};
mod config;

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
                    config_path = Some(String::from(local_config_path.to_str().unwrap()));
                }
            }
        }
    }

    // Check config directory for config file
    if config_path.is_none() {
        // TODO
    }

    let config = config::Config::parse(
        &fs::read_to_string(config_path.expect("could not detect configuration file"))
            .expect("could not load configuration file"),
    )
    .expect("could not parse configuration file");

    let profile = config.default_profile().expect("default profile not found");

    let mut user_args: Vec<String> = env::args().collect();
    user_args.remove(0);

    let mut config_args: Vec<String> = Vec::new();

    match profile.user_name() {
        Some(n) => {
            config_args.push("-c".to_string());
            config_args.push(format!("user.name={}", n));
        }
        None => {}
    }
    match profile.user_email() {
        Some(e) => {
            config_args.push("-c".to_string());
            config_args.push(format!("user.email={}", e));
        }
        None => {}
    }
    match profile.user_signingkey() {
        Some(s) => {
            config_args.push("-c".to_string());
            config_args.push(format!("user.signingkey={}", s));
        }
        None => {}
    }
    match profile.commit_gpgsign() {
        Some(g) => {
            config_args.push("-c".to_string());
            config_args.push(format!("commit.gpgsign={}", g));
        }
        None => {}
    }
    match profile.tag_gpgsign() {
        Some(g) => {
            config_args.push("-c".to_string());
            config_args.push(format!("tag.gpgsign={}", g));
        }
        None => {}
    }
    match profile.pull_rebase() {
        Some(r) => {
            config_args.push("-c".to_string());
            config_args.push(format!("pull.rebase={}", r));
        }
        None => {}
    }
    match profile.sshkey() {
        Some(s) => {
            config_args.push("-c".to_string());
            config_args.push(format!("core.sshCommand=ssh -i '{}'", s));
        }
        None => {}
    }

    Command::new("git")
        .args(config_args)
        .args(user_args)
        .status()
        .expect("failed to execute git command");
}
