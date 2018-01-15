use toml;

use cli;

use std::env;
use std::io::Read;
use std::fs::File;
use std::path::{Path, PathBuf};

mod errors {
    use toml;
    use std::io;

    error_chain! {
        foreign_links {
            Toml(toml::de::Error);
            Io(io::Error);
        }
    }
}
pub use self::errors::{Result, Error, ErrorKind};


#[inline]
pub fn load_config() -> Config {
    let mut userpath = env::home_dir().unwrap();
    userpath.push(".config/tr1pd.toml");

    let globalpath = PathBuf::from("/etc/tr1pd/tr1pd.toml");

    for path in &[userpath, globalpath] {
        if let Ok(config) = load_configfile(&path) {
            info!("using config from {:?}", path);
            return config;
        }
    }

    info!("using default config");
    Config::default()
}

#[inline]
pub fn load_configfile<P: AsRef<Path>>(path: P) -> Result<Config> {
    let mut file = File::open(path)?;

    let mut buf = String::new();
    file.read_to_string(&mut buf)?;

    let config = Config::parse(&buf)?;
    Ok(config)
}


#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub daemon: DaemonConfig,
    #[serde(default)]
    pub security: SecurityConfig,
}

impl Config {
    #[inline]
    pub fn parse(data: &str) -> Result<Config> {
        let config = toml::from_str(data)?;
        Ok(config)
    }

    #[inline]
    pub fn socket(&self) -> &str {
        match self.daemon.socket.as_ref() {
            Some(socket) => socket,
            None => cli::TR1PD_SOCKET,
        }
    }

    #[inline]
    pub fn set_socket<I: Into<String>>(&mut self, socket: Option<I>) {
        if let Some(socket) = socket {
            self.daemon.socket = Some(socket.into());
        }
    }

    #[inline]
    pub fn datadir(&self) -> &str {
        match self.daemon.datadir.as_ref() {
            Some(datadir) => datadir,
            None => cli::TR1PD_DATADIR,
        }
    }

    #[inline]
    pub fn set_datadir<I: Into<String>>(&mut self, datadir: Option<I>) {
        if let Some(datadir) = datadir {
            self.daemon.datadir = Some(datadir.into());
        }
    }

    #[inline]
    pub fn pub_key(&self) -> &str {
        match self.daemon.pub_key.as_ref() {
            Some(pub_key) => pub_key,
            None => "/etc/tr1pd/lt.pk",
        }
    }

    #[inline]
    pub fn sec_key(&self) -> &str {
        match self.daemon.sec_key.as_ref() {
            Some(sec_key) => sec_key,
            None => "/etc/tr1pd/lt.sk",
        }
    }
}

#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct DaemonConfig {
    pub datadir: Option<String>,
    pub socket: Option<String>,

    pub pub_key: Option<String>,
    pub sec_key: Option<String>,
}

#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SecurityConfig {
    #[serde(default)]
    pub strict_chroot: bool,
}
