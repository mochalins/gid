use serde::Deserialize;
use std::{
    collections::HashMap,
    env, fs,
    io::Write,
    path::{Path, PathBuf},
    str,
};
use toml::from_str;

#[derive(Deserialize, Debug)]
pub struct Config {
    active: String,

    #[serde(flatten)]
    profiles: HashMap<String, Profile>,
}

#[derive(Deserialize, Debug)]
pub struct Profile {
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
            let local_config_path = env::current_exe();
            if let Ok(p) = local_config_path {
                let mut local_config_path = p;
                local_config_path.set_file_name("gid.toml");
                if let Ok(true) = local_config_path.try_exists() {
                    config_path = Some(local_config_path);
                }
            }
        }

        // Check config directory for config file
        if config_path.is_none() {
            let home_env = if env::consts::OS == "windows" {
                "USERPROFILE"
            } else {
                "HOME"
            };
            let home_path = env::var(home_env);
            if let Ok(s) = home_path {
                let mut home_config_path = PathBuf::from(&s);
                home_config_path.push(".config");
                home_config_path.push("gid");
                home_config_path.push("gid.toml");
                if let Ok(true) = home_config_path.try_exists() {
                    config_path = Some(home_config_path);
                }
            }
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

    pub fn list_profile_names(&self) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();

        for (name, _) in &self.profiles {
            result.push(name.to_string());
        }

        result
    }

    pub fn get_profile<'a>(&'a self, profile: &str) -> Option<&'a Profile> {
        self.profiles.get(profile)
    }

    pub fn set_active(&mut self, profile: &str) -> Result<(), ()> {
        if self.get_profile(profile).is_some() {
            self.active = profile.to_string();
            return Ok(());
        }
        Err(())
    }

    pub fn get_active_profile_name(&self) -> Option<String> {
        if let Some(_) = self.get_active_profile() {
            return Some(self.active.clone());
        }
        None
    }

    pub fn get_active_profile<'a>(&'a self) -> Option<&'a Profile> {
        self.profiles.get(&self.active)
    }

    pub fn to_string(&self) -> String {
        let mut result = Vec::new();
        writeln!(&mut result, "active = \"{}\"", self.active).unwrap();

        for (name, profile) in &self.profiles {
            writeln!(&mut result, "").unwrap();
            writeln!(&mut result, "[{}]", name).unwrap();
            write!(&mut result, "{}", profile.to_string()).unwrap();
        }

        str::from_utf8(&result).unwrap().to_string()
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

    pub fn to_string(&self) -> String {
        let mut result = Vec::new();

        if let Some(v) = self.user_name() {
            writeln!(&mut result, "user.name = \"{}\"", v).unwrap();
        }
        if let Some(v) = self.user_email() {
            writeln!(&mut result, "user.email = \"{}\"", v).unwrap();
        }
        if let Some(v) = self.user_signingkey() {
            writeln!(&mut result, "user.signingkey = \"{}\"", v).unwrap();
        }
        if let Some(v) = self.commit_gpgsign() {
            writeln!(&mut result, "commit.gpgsign = {}", v).unwrap();
        }
        if let Some(v) = self.tag_gpgsign() {
            writeln!(&mut result, "tag.gpgsign = {}", v).unwrap();
        }
        if let Some(v) = self.pull_rebase() {
            writeln!(&mut result, "pull.rebase = {}", v).unwrap();
        }
        if let Some(v) = self.sshkey() {
            writeln!(&mut result, "sshkey = \"{}\"", v).unwrap();
        }

        str::from_utf8(&result).unwrap().to_string()
    }
}
