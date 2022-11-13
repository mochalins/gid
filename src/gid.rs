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
        if let Ok(p) = local_config_path {
            let mut local_config_path = p;
            local_config_path.push("gid.toml");
            if let Ok(true) = local_config_path.try_exists() {
                config_path = local_config_path.to_str().map(str::to_string);
            }
        }
    }

    // Check config directory for config file
    if config_path.is_none() {
        // TODO
    }

    let config_path = config_path.expect("could not detect configuration file");

    let config = config::Config::parse(
        &fs::read_to_string(config_path).expect("could not load configuration file"),
    )
    .expect("could not parse configuration file");

    let profile = config.default_profile().expect("default profile not found");

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
        config_args.push(format!("core.sshCommand=ssh -i '{}'", s));
    }

    Command::new("git")
        .args(config_args)
        .args(user_args)
        .status()
        .expect("failed to execute git command");
}
