use toml::Value;
use std::path::PathBuf;

use crate::file::config_dir;
use crate::profile::profile;


// Structure config
// [default]
// clone_path = "/Users/seka/projects/mttrbit/bardo-repos"
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

pub fn consume_repos2<F>(f: F) where
    F: Fn(&str, &Value)
{
    let default_profile = profile().unwrap();
    config_file().map(|path_buf| {
        crate::file::read_toml(path_buf).map(|content| {
            let sections = &content[default_profile];
            let path = &sections["clone_path"].as_str().unwrap();
            let repos = &sections["repositories"];
            for repo in repos.as_array().unwrap() {
                f(path, repo);
            };
        })
    });
}

#[cfg(test)]
mod config_tests {
    use super::*;
    use std::env;
    use std::process::Command;

    #[test]
    fn test_command() {
        let clone_repo = |path: &str, repo: &Value| {
            let org = repo.get("org").unwrap();
            let name = repo.get("name").unwrap();
            let ssh_url = format!("git@github.com:{}/{}.git",
                                     org.as_str().unwrap(),
                                     name.as_str().unwrap());

            let status = Command::new("sh")
                .current_dir(path)
                .arg("-c")
                .arg(format!("git clone {}", ssh_url))
                .status()
                .expect("failed to execute process");

            println!("process exited with: {}", status);
        };

        consume_repos2(clone_repo);
    }

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
    fn test_print_path() {
        let print_path = |path: &str, _repo: &Value| {
            println!("{}", path)
        };

        consume_repos2(print_path);
    }

    #[test]
    fn test_print_urls() {
        let print_url = |repo: &Value| {
            let org = repo.get("org").unwrap();
            let name = repo.get("name").unwrap();
            let url = format!("https://github.com/{}/{}",
                                     org.as_str().unwrap(),
                                     name.as_str().unwrap());
            assert_eq!(url, "https://github.com/crvshlab/backoffice-is-analysis");
        };

        consume_repos(print_url);
    }

    #[test]
    fn test_print_ssh_urls() {
        let print_url = |repo: &Value| {
            let org = repo.get("org").unwrap();
            let name = repo.get("name").unwrap();
            let url = format!("git@github.com:{}/{}.git",
                                     org.as_str().unwrap(),
                                     name.as_str().unwrap());
            assert_eq!(url, "git@github.com:crvshlab/backoffice-is-analysis.git");
        };

        consume_repos(print_url);
    }

    #[test]
    fn test_repository_file() {
        assert_eq!(config_file(), Some(PathBuf::from("/Users/seka/.config/bardo/gh/config")));

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
        let _profile = &decoded["default"];
        // println!("repo file: {:?}", profile["repositories"]);
    }

    #[test]
    fn test_config_dir() {
        config_dir().map(|buf| {
            assert_eq!(buf.as_path().to_str(), Some("/Users/seka/.config/bardo/gh"))
        });
    }
}
