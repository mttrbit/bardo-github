
use std::env;
use toml::Value;
use std::path::PathBuf;

use crate::file::{config_dir, project_dir, read};
use crate::profile::profile;

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

pub fn consume_repos<F>(f: F) where
    F: Fn(&Value)
{
    let default_profile = profile().unwrap();
    config_file().map(|path_buf| {
        crate::file::read_toml(path_buf).map(|content| {
            let sections = &content[default_profile];
            let repos = &sections["repositories"];
            for repo in repos.as_array().unwrap() {
                f(repo);
            };
        })
    });
}

#[cfg(test)]
mod config_tests {
    use super::*;

    #[test]
    fn test_repos_properties() {
        let print_details = |repo: &Value| {
            assert_eq!(repo.get("org").is_some(), true);
            assert_eq!(repo.get("name").is_some(), true);
            assert_eq!(repo.get("regex").is_none(), true);
        };

        consume_repos(print_details);
    }

    #[test]
    fn test_print_urls() {
        let print_url = |repo: &Value| {
            let org = repo.get("org").unwrap();
            let name = repo.get("name").unwrap();
            let github_url = format!("https://github.com/{}/{}",
                                     org.as_str().unwrap(),
                                     name.as_str().unwrap());
            assert_eq!(github_url, "https://github.com/crvshlab/backoffice-is-analysis");
        };

        consume_repos(print_url);
    }

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
        // println!("repo file: {:?}", profile["repositories"]);
    }

    #[test]
    fn test_config_dir() {
        config_dir().map(|buf| {
            assert_eq!(buf.as_path().to_str(), Some("/Users/seka/.config/bardo/gh"))
        });
    }
}
