use std::io;
use std::path::PathBuf;

use crate::credentials::{BardoCredentials, credentials_file};
use crate::config::{BardoConfig, config_file};
use crate::file::read_toml;

pub use std::io::Result;

pub struct BardoContext {
    profile: String,
    credentials: BardoCredentials,
    config: BardoConfig,
}

impl BardoContext {

    pub fn profile(&self) -> &String {
        &self.profile
    }

    pub fn profile_mut(&mut self) -> &mut String {
        &mut self.profile
    }

    pub fn credentials(&self) -> &BardoCredentials {
        &self.credentials
    }

    pub fn config(&self) -> &BardoConfig {
        &self.config
    }

    pub fn init(profile: &str) -> Result<Self> {
        let toml_reader = |buf: PathBuf| read_toml(buf.as_path());

        let creds_reader = || credentials_file()
            .ok_or(io::Error::new(io::ErrorKind::InvalidData, "cannot read credentials file"))
            .and_then(toml_reader);

        let config_reader = || config_file()
            .ok_or(io::Error::new(io::ErrorKind::InvalidData, "cannot read config file"))
            .and_then(toml_reader);

        let credentials: Result<BardoCredentials> = BardoCredentials::read_from(creds_reader);
        let config: Result<BardoConfig> = BardoConfig::read_from(config_reader);

        match (credentials, config) {
            (Ok(a), Ok(b)) => Ok(Self {
                credentials: a,
                config: b,
                profile: profile.to_string(),
            }),
            _ => Err(io::Error::new(io::ErrorKind::InvalidData, "error while initializing context")),
        }
    }
}
