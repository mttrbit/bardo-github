
use std::env;
use toml::Value;
use std::path::PathBuf;

use crate::file::{config_dir, project_dir, read};

// Structure config
// [default]
// clone_path = "/users/seka/projects/mttrbit/bardo-repos"
// repositories = [
//   {org = "crvshlab", name = "test"}
// , {org = "crvshlab", regex = "nodejs-*"}
// ]
pub fn config_file() -> Option<PathBuf> {
    config_dir().map(|h| h.join("config"))
}


#[cfg(test)]
mod config_tests {
    use super::*;

    #[test]
    fn test_repository_file() {
        assert_eq!(config_file(), Some(PathBuf::from("/Users/seka/.config/bardo/gh/config")));
    }

    #[test]
    fn test_set_env() {
        env::set_var("BARDO_CONFIG_HOME", "/Users/seka/foobar");
        assert_eq!(config_file(), Some(PathBuf::from("/Users/seka/foobar/gh/config")));
        env::remove_var("BARDO_CONFIG_HOME");
    }

    #[test]
    fn test_config() {
        let toml_str = r#"[default]
clone_path = "/Users/seka/projects/mttrbit/bardo-repos"
repositories = [
  {org = "crvshlab", name = "test"}
, {org = "crvshlab", regex = "nodejs-*"}
]
"#;

        let decoded: Value = toml::from_str(toml_str).unwrap();
        let profile = &decoded["default"];
        println!("repo file: {:?}", profile["repositories"]);
    }

    #[test]
    fn test_config_dir() {
        config_dir().map(|buf| {
            assert_eq!(buf.as_path().to_str(), Some("/Users/seka/.config/bardo/gh"))
        });
    }
}
