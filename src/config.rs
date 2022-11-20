use serde::de::Error;
use std::{
    borrow::Borrow,
    cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd},
    collections::{BTreeMap, BTreeSet},
    env, fs,
    io::Write,
    path::{Path, PathBuf},
    str::{from_utf8, FromStr},
    string::ToString,
};
use toml;

pub trait ToGitString {
    fn to_git_string(&self) -> String;
}

pub trait FromGitStr {
    fn from_git_str(s: &str) -> Result<Self, String>
    where
        Self: Sized;
}

#[derive(Debug)]
pub struct Config {
    pub active: Option<String>,
    pub profiles: BTreeSet<Profile>,
}

impl Config {
    /// Get path of config file in executable directory, if path exists.
    pub fn exe_path() -> Option<PathBuf> {
        let config_path = env::current_exe();
        if let Ok(p) = config_path {
            let mut config_path = p;
            config_path.set_file_name("gid.toml");
            return Some(config_path);
        }
        None
    }

    /// Get path of config file in user config directory, if path exists.
    pub fn config_path() -> Option<PathBuf> {
        let home_env = if env::consts::OS == "windows" {
            "USERPROFILE"
        } else {
            "HOME"
        };
        let home_path = env::var(home_env);
        if let Ok(s) = home_path {
            let mut config_path = PathBuf::from(&s);
            config_path.push(".config");
            config_path.push("gid");
            config_path.push("gid.toml");
            return Some(config_path);
        }
        None
    }

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

        // Check current executable directory for config file
        if config_path.is_none() {
            let local_config_path = Config::exe_path();
            if let Some(ref p) = local_config_path {
                if let Ok(true) = p.try_exists() {
                    config_path = local_config_path;
                }
            }
        }

        // Check user config directory for config file
        if config_path.is_none() {
            let config_config_path = Config::config_path();
            if let Some(ref p) = config_config_path {
                if let Ok(true) = p.try_exists() {
                    config_path = config_config_path;
                }
            }
        }
        config_path
    }

    pub fn parse_file(p: &Path) -> Result<Self, String> {
        let config_string = fs::read_to_string(p);
        if config_string.is_err() {
            return Err("could not load configuration file".to_string());
        }
        let config_string = config_string.unwrap();

        match config_string.parse::<Config>() {
            Ok(res) => Ok(res),
            Err(e) => Err(e.to_string()),
        }
    }
}

impl FromStr for Config {
    type Err = toml::de::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s_parse = s.parse::<toml::Value>();
        match s_parse {
            Ok(mut v) => {
                if let Some(s_table) = v.as_table_mut() {
                    let mut result = Config {
                        active: None,
                        profiles: BTreeSet::new(),
                    };

                    // Parse active profile
                    if let Some(s) = s_table.remove("active") {
                        if let Some(sv) = s.as_str() {
                            result.active = Some(String::from(sv));
                        }
                    }

                    // Parse profiles
                    for (name, profile) in s_table.iter() {
                        if let Some(pt) = profile.as_table() {
                            let mut result_profile: Profile = Profile {
                                name: name.to_string(),
                                fields: BTreeMap::new(),
                            };
                            let mut field_queue: Vec<(String, &toml::Value)> = Vec::new();
                            for (key, val) in pt.iter() {
                                field_queue.push((key.to_string(), val));
                            }
                            while field_queue.len() > 0 {
                                let (key, val) = field_queue.pop().unwrap();
                                match val {
                                    toml::Value::Table(t) => {
                                        for (tkey, tval) in t.iter() {
                                            field_queue.push((format!("{}.{}", key, tkey), tval));
                                        }
                                    }
                                    toml::Value::Array(a) => {
                                        let mut color_array: Vec<Color> = Vec::new();
                                        for c in a.iter() {
                                            match c {
                                                toml::Value::String(s) => {
                                                    color_array.push(Color::String(s.to_string()));
                                                }
                                                toml::Value::Integer(i) => {
                                                    color_array.push(Color::Number(*i as u32));
                                                }
                                                _ => {
                                                    return Err(toml::de::Error::custom(
                                                        "configuration cannot \
                                                    contain non-color arrays \
                                                    (colors are 0-255, 24 bit \
                                                     hex codes, or color name \
                                                     and attribute strings as \
                                                     defined by Git \
                                                     configuration values",
                                                    ));
                                                }
                                            }
                                        }
                                        result_profile
                                            .fields
                                            .insert(key, Value::ColorArray(color_array));
                                    }
                                    toml::Value::Boolean(b) => {
                                        result_profile.fields.insert(key, Value::Boolean(*b));
                                    }
                                    toml::Value::Integer(i) => {
                                        result_profile.fields.insert(key, Value::Integer(*i));
                                    }
                                    toml::Value::String(s) => {
                                        result_profile
                                            .fields
                                            .insert(key, Value::String(s.to_string()));
                                    }
                                    _ => {
                                        return Err(toml::de::Error::custom(
                                            "unknown Git representation for \
                                            TOML float, date, time, and \
                                            datetime types",
                                        ));
                                    }
                                }
                            }
                            result.profiles.insert(result_profile);
                        }
                    }
                    return Ok(result);
                } else {
                    return Err(toml::de::Error::custom(
                        "config is not a top level table".to_string(),
                    ));
                }
            }
            Err(e) => Err(e),
        }
    }
}

impl ToString for Config {
    fn to_string(&self) -> String {
        let mut result = Vec::new();
        if let Some(s) = &self.active {
            writeln!(&mut result, "active = \"{}\"", s).unwrap();
        }
        for profile in self.profiles.iter() {
            writeln!(&mut result, "").unwrap();
            write!(&mut result, "{}", profile.to_string()).unwrap();
        }
        from_utf8(&result).unwrap().to_string()
    }
}

#[derive(Debug)]
pub struct Profile {
    pub name: String,
    pub fields: BTreeMap<String, Value>,
}

impl Borrow<str> for Profile {
    fn borrow(&self) -> &str {
        &self.name
    }
}

impl Borrow<String> for Profile {
    fn borrow(&self) -> &String {
        &self.name
    }
}

impl Eq for Profile {}

impl Ord for Profile {
    fn cmp(&self, other: &Self) -> Ordering {
        self.name.cmp(&other.name)
    }
}

impl PartialEq for Profile {
    fn eq(&self, other: &Self) -> bool {
        self.name.eq(&other.name)
    }
}

impl PartialOrd for Profile {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.name.partial_cmp(&other.name)
    }
}

impl ToString for Profile {
    fn to_string(&self) -> String {
        let mut result = Vec::new();
        writeln!(&mut result, "[{}]", self.name).unwrap();
        for (key, val) in self.fields.iter() {
            writeln!(&mut result, "{} = {}", key, val.to_string()).unwrap();
        }
        from_utf8(&result).unwrap().to_string()
    }
}

#[derive(Debug)]
pub enum Value {
    Boolean(bool),
    ColorArray(Vec<Color>),
    Integer(i64),
    String(String),
}

impl FromGitStr for Value {
    fn from_git_str(s: &str) -> Result<Self, String> {
        if let Ok(b) = s.parse::<bool>() {
            return Ok(Self::Boolean(b));
        } else if let Ok(i) = s.parse::<i64>() {
            return Ok(Self::Integer(i));
        } else {
            return Ok(Self::String(s.to_string()));
        }
    }
}

impl ToGitString for Value {
    fn to_git_string(&self) -> String {
        match self {
            Self::Boolean(b) => b.to_string(),
            Self::ColorArray(v) => v
                .iter()
                .map(|c| c.to_git_string())
                .collect::<Vec<_>>()
                .join(" "),
            Self::Integer(i) => i.to_string(),
            Self::String(s) => s.to_string(),
        }
    }
}

impl ToString for Value {
    fn to_string(&self) -> String {
        match self {
            Self::Boolean(b) => b.to_string(),
            Self::ColorArray(v) => format!(
                "[{}]",
                v.iter()
                    .map(|c| c.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            Self::Integer(i) => i.to_string(),
            Self::String(s) => {
                if s.contains("\n") {
                    format!(
                        "\"\"\"{}\"\"\"",
                        s.replace("\\", "\\\\")
                            .replace("\"", "\\\"")
                            .replace("\t", "\\t")
                    )
                } else {
                    format!(
                        "\"{}\"",
                        s.replace("\\", "\\\\")
                            .replace("\"", "\\\"")
                            .replace("\t", "\\t")
                    )
                }
            }
        }
    }
}

#[derive(Debug)]
pub enum Color {
    Number(u32),
    String(String),
}

impl ToGitString for Color {
    fn to_git_string(&self) -> String {
        match self {
            Self::Number(i) => {
                if *i < 256 {
                    i.to_string()
                } else {
                    format!("#{:x}", i)
                }
            }
            Self::String(s) => s.to_string(),
        }
    }
}

impl ToString for Color {
    fn to_string(&self) -> String {
        match self {
            Self::Number(i) => {
                if *i < 256 {
                    i.to_string()
                } else {
                    format!("0x{:x}", i)
                }
            }
            Self::String(s) => format!("\"{}\"", s),
        }
    }
}
