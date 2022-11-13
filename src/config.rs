use serde::Deserialize;
use std::{
    collections::HashMap,
    env, fs,
    path::{Path, PathBuf},
};
use toml::from_str;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub active: String,
    pub profile: HashMap<String, Profile>,
}

#[derive(Deserialize, Debug)]
pub struct Profile {
    pub user: Option<ProfileUser>,
    pub commit: Option<ProfileCommit>,
    pub tag: Option<ProfileTag>,
    pub pull: Option<ProfilePull>,
    pub sshkey: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct ProfileUser {
    pub name: Option<String>,
    pub email: Option<String>,
    pub signingkey: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct ProfileCommit {
    pub gpgsign: Option<bool>,
}

#[derive(Deserialize, Debug)]
pub struct ProfileTag {
    pub gpgsign: Option<bool>,
}

#[derive(Deserialize, Debug)]
pub struct ProfilePull {
    pub rebase: Option<bool>,
}

impl Config {
    pub fn detect() -> Option<PathBuf> {
        let mut config_path: Option<PathBuf> = None;

        // Check environment variable for config path
        let env_config_path = env::var("GID_CONFIG");
        if let Ok(s) = env_config_path {
            let env_config_path = PathBuf::from(&s);
            if let Ok(true) = env_config_path.try_exists() {
                config_path = Some(env_config_path);
            }
        }

        // Check local working directory for config file
        if config_path.is_none() {
            let local_config_path = env::current_dir();
            if let Ok(p) = local_config_path {
                let mut local_config_path = p;
                local_config_path.push("gid.toml");
                if let Ok(true) = local_config_path.try_exists() {
                    config_path = Some(local_config_path);
                }
            }
        }

        // Check config directory for config file
        if config_path.is_none() {
            // TODO
        }
        config_path
    }

    pub fn parse(s: &str) -> Result<Self, String> {
        match from_str(s) {
            Ok(c) => Ok(c),
            Err(e) => Err(e.to_string()),
        }
    }

    pub fn parse_file(p: &Path) -> Result<Self, String> {
        let config_string = fs::read_to_string(p);
        if config_string.is_err() {
            return Err("could not load configuration file".to_string());
        }
        let config_string = config_string.unwrap();

        Config::parse(&config_string)
    }

    pub fn get_profile<'a>(&'a self, profile: &str) -> Option<&'a Profile> {
        self.profile.get(profile)
    }

    pub fn set_active(&mut self, profile: &str) -> Result<(), ()> {
        if self.get_profile(profile).is_some() {
            self.active = profile.to_string();
            return Ok(());
        }
        Err(())
    }

    pub fn get_active_profile<'a>(&'a self) -> Option<&'a Profile> {
        self.profile.get(&self.active)
    }
}

impl Profile {
    pub fn user_name(&self) -> Option<String> {
        match &self.user {
            Some(u) => u.name.as_ref().cloned(),
            None => None,
        }
    }

    pub fn user_email(&self) -> Option<String> {
        match &self.user {
            Some(u) => u.email.as_ref().cloned(),
            None => None,
        }
    }

    pub fn user_signingkey(&self) -> Option<String> {
        match &self.user {
            Some(u) => u.signingkey.as_ref().cloned(),
            None => None,
        }
    }

    pub fn commit_gpgsign(&self) -> Option<bool> {
        match &self.commit {
            Some(c) => c.gpgsign.as_ref().cloned(),
            None => None,
        }
    }

    pub fn tag_gpgsign(&self) -> Option<bool> {
        match &self.tag {
            Some(t) => t.gpgsign.as_ref().cloned(),
            None => None,
        }
    }

    pub fn pull_rebase(&self) -> Option<bool> {
        match &self.pull {
            Some(p) => p.rebase.as_ref().cloned(),
            None => None,
        }
    }

    pub fn sshkey(&self) -> Option<String> {
        self.sshkey.as_ref().cloned()
    }
}
