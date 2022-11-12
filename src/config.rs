use serde::Deserialize;
use std::collections::HashMap;
use toml::from_str;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub default: String,
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
    pub fn parse(s: &str) -> Result<Self, String> {
        match from_str(s) {
            Ok(c) => Ok(c),
            Err(e) => Err(e.to_string()),
        }
    }

    pub fn profile<'a>(&'a self, profile: &str) -> Option<&'a Profile> {
        self.profile.get(profile)
    }

    pub fn default_profile<'a>(&'a self) -> Option<&'a Profile> {
        self.profile.get(&self.default)
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
