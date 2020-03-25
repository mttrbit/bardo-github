use std::collections::HashMap;
use std::env;
use std::io;
use std::path::Path;
use std::path::PathBuf;

use serde_derive::Deserialize;

use crate::file::{config_dir, read};
use crate::profile::profile;

pub use io::Result;

#[derive(Debug, Deserialize)]
pub struct Profiles {
    profiles: HashMap<String, ProfileConfig>,
}

#[derive(Debug, Deserialize)]
pub struct ProfileConfig {
    bardo_client_id: Option<String>,
    bardo_client_secret: Option<String>,
    bardo_access_token: Option<String>,
}

impl Clone for ProfileConfig {
    fn clone(&self) -> Self {
        Self {
            bardo_client_id: self.bardo_client_id.clone(),
            bardo_client_secret: self.bardo_client_secret.clone(),
            bardo_access_token: self.bardo_access_token.clone(),
        }
    }
}

impl ProfileConfig {
    pub fn new(client_id: String, client_secret: String, access_token: String) -> Self {
        Self {
            bardo_client_id: Some(client_id),
            bardo_client_secret: Some(client_secret),
            bardo_access_token: Some(access_token),
        }
    }
}

// Structure credentials
// [default]
// bardo_github_client_id =
// bardo_github_client_secret =
// bardo_github_access_token =
pub fn credentials_file() -> Option<PathBuf> {
    config_dir().map(|h| h.join("credentials"))
}

/// Read toml file into `Value` from given path.
/// The path can be `String` or `Path`.
pub fn read_toml<P: AsRef<Path>>(path: P) -> Result<Profiles> {
    let bytes: Vec<u8> = read(path)?;
    toml::from_slice(bytes.as_ref())
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "file content is no valid toml"))
}

pub fn read_toml_str(toml_str: &str) -> Result<Profiles> {
    toml::from_slice(toml_str.as_bytes())
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "file content is no valid toml"))
}

pub fn write_access_token(access_token: &str) -> Result<()> {
    let default_profile = profile().unwrap();

    match credentials_file().map(|path_buf| {
        crate::file::read_toml(path_buf.clone())
            .map(|mut content| {
                content["profiles"][default_profile]["bardo_access_token"] = toml::Value::from(access_token.to_string());
                let toml = toml::to_string(&content).unwrap();
                crate::file::write_str(path_buf.as_path(), toml)
            })
    }) {
        Some(_) => Ok(()),
        None => Err(io::Error::new(io::ErrorKind::InvalidData, "invalid toml")),
    }
}

fn read_from_credentials_file<F>(f: F) -> Option<String> where
    F: Fn(&ProfileConfig) -> Option<String>
{
    env::var_os("GITHUB_CLIENT_ID")
        .map(|s| std::ffi::OsString::into_string(s).unwrap())
        .or_else(|| match credentials_file() {
            Some(buf) => {
                let toml_str = buf.as_path().to_str().unwrap();
                let creds: Profiles = read_toml(toml_str).unwrap();
                let default_profile = profile().unwrap();
                match creds.profiles.get(&default_profile) {
                    Some(p) => f(&p),
                    _ => panic!("profile not found"),
                }
            }
            None => panic!("could not read from config file"),
        })
}

fn reader_test<F>(f: F) -> Option<ProfileConfig> where
    F: Fn(&ProfileConfig) -> Option<ProfileConfig>
{
    match credentials_file() {
            Some(buf) => {
                let toml_str = buf.as_path().to_str().unwrap();
                let creds: Profiles = read_toml(toml_str).unwrap();
                let default_profile = profile().unwrap();
                match creds.profiles.get(&default_profile) {
                    Some(p) => f(&p),
                    _ => panic!("profile not found"),
                }
            }
            None => panic!("could not read from config file"),
    }
}

fn read_pc(pc: &ProfileConfig) -> Option<ProfileConfig> {
    Some(pc.clone())
}

fn read_client_id(pc: &ProfileConfig) -> Option<String> {
    let client_id = pc.bardo_client_id.as_ref().unwrap();
    Some(client_id.to_string())
}

fn read_client_secret(pc: &ProfileConfig) -> Option<String> {
    let client_secret = pc.bardo_client_secret.as_ref().unwrap();
    Some(client_secret.to_string())
}

fn read_access_token(pc: &ProfileConfig) -> Option<String> {
    let access_token = pc.bardo_access_token.as_ref().unwrap();
    Some(access_token.to_string())
}

pub fn client_id() -> Option<String> {
    read_from_credentials_file(read_client_id)
}

pub fn client_secret() -> Option<String> {
    read_from_credentials_file(read_client_secret)
}

pub fn access_token() -> Option<String> {
    read_from_credentials_file(read_access_token)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reader() {
        println!("{:?}", reader_test(read_pc));
    }

    #[test]
    fn test_load_toml() {
        let toml_str = r#"
            [profiles.default]
            bardo_client_id = "client_id"
            bardo_client_secret = "client_secret"
            bardo_access_token = "access_token"
        "#;

        let p: Profiles = read_toml_str(toml_str).ok().unwrap();
        match p.profiles.get("default") {
            Some(default) => {
                assert_eq!(default.bardo_client_id, Some("client_id".to_string()));
                assert_eq!(
                    default.bardo_client_secret,
                    Some("client_secret".to_string())
                );
                assert_eq!(default.bardo_access_token, Some("access_token".to_string()));
            }
            _ => panic!("profile not found"),
        };
    }

    #[test]
    fn test_load_toml_fails() {
        let toml_str = r#"
            [profiles.default]
            bardo_client_id = "client_id"
            bardo_client_secret = "client_secret"
            bardo_access_token = "access_token"
        "#;

        let p: Profiles = read_toml_str(toml_str).ok().unwrap();
        assert_eq!(p.profiles.get("default2").is_none(), true);
    }

    // #[test]
    // fn test_write_access_token() {
    //     write_access_token("hello".to_string());
    // }


    // #[test]
//     fn test_write_credential_file() {
//         project_dir().map(|path| {
//             env::set_var("BARDO_CONFIG_HOME", path.join("temp"));
//         });
//         let toml_str = r#"[default]
// bardo_github_client_id = "client_id"
// bardo_github_client_secret = "client_secret"
// bardo_github_access_token = "access_token"
// "#;

//         write_config_dir();
//         match credentials_file() {
//             Some(buf) => {
//                 let _ = write_str(buf.as_path(), toml_str).map_err(|_| {
//                     panic!("could not write config file");
//                 });
//             },
//             None => { panic!("could not read path to config file"); }
//         }
//     }
}
