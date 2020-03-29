use std::collections::HashMap;
use std::io;
use toml::Value;
use std::path::PathBuf;

use crate::file::config_dir;
use crate::profile::profile;

pub use io::Result;

#[derive(Debug)]
pub struct Org(pub String);
#[derive(Debug)]
pub struct Name(pub String);
#[derive(Debug)]
pub struct Regex(pub String);
#[derive(Debug)]
pub struct ClonePath(pub String);

#[derive(Debug)]
pub struct Repository {
    org: Org,
    name: Option<Name>,
    regex: Option<Regex>,
}

#[derive(Debug)]
pub struct Repositories(pub Vec<Repository>);

#[derive(Debug)]
pub struct Configuration {
    clone_path: ClonePath,
    repositories: Repositories,
}

pub struct BardoConfig {
    profiles: HashMap<String, Configuration>,
}

// Structure config
// [default]
// clone_path = "/Users/seka/projects/mttrbit/bardo-repos"
// repositories = [
//   {org = "crvshlab", name = "test"}
// , {org = "crvshlab", regex = "nodejs-*"}
// ]
impl Repository {

    pub fn new(org: Org, name: Option<Name>, regex: Option<Regex>) -> Self {
        Self {
            org: org,
            name: name,
            regex: regex,
        }
    }

    pub fn org(&self) -> &Org {
        &self.org
    }

    pub fn org_mut(&mut self) -> &mut Org {
        &mut self.org
    }

    pub fn name(&self) -> Option<&Name> {
        self.name.as_ref()
    }

    pub fn name_mut(&mut self) -> &mut Option<Name> {
        &mut self.name
    }

    pub fn regex(&self) -> Option<&Regex> {
        self.regex.as_ref()
    }

    pub fn regex_mut(&mut self) -> &mut Option<Regex> {
        &mut self.regex
    }

    pub fn read_from<F>(reader: F) -> Result<Self>
    where
        F: Fn() -> Result<Value>,
    {
        reader().and_then(|repo| {
            let str_org = repo["org"].as_str().expect("field 'org' is missing");
            let org = Org(str_org.to_string());

            let opt_name = repo.get("name");
            let opt_regex = repo.get("regex");

            match opt_name.xor(opt_regex) {
                Some(_) => {
                    let name = opt_name.map(|f| Name(f.as_str().unwrap().to_string()));
                    let regex = opt_regex.map(|f| Regex(f.as_str().unwrap().to_string()));
                    Ok(Self {
                        org: org,
                        name: name,
                        regex: regex,
                    })
                },
                None => Err(io::Error::new(io::ErrorKind::InvalidData, "set either 'name' or 'regex'")),
            }
        })
    }
}

impl Repositories {
    pub fn read_from<F>(reader: F) -> Result<Repositories>
    where
        F: Fn() -> Result<Value>,
    {
        reader().and_then(|repositories| {
            let repos = repositories.as_array().expect("field 'repositories' is missing");
            let mut vec_repos = Vec::with_capacity(repos.len());

            for (i, r) in repos.iter().enumerate() {
                let res = Repository::read_from(|| Ok(r.clone())).map(|r| vec_repos.insert(i, r));

                if res.is_err() {
                    return Err(res.unwrap_err());
                }
            }

            Ok(Repositories(vec_repos))
        })
    }

    pub fn add(&mut self, repository: Repository) {
        self.0.insert(self.0.len(), repository);
    }
}

impl Configuration {

    pub fn clone_path(&self) -> &ClonePath {
        &self.clone_path
    }

    pub fn repositories(&self) -> &Repositories {
        &self.repositories
    }

    pub fn repositories_mut(&mut self) -> &mut Repositories {
        &mut self.repositories
    }

    pub fn read_from<F>(reader: F) -> Result<Self>
    where
        F: Fn() -> Result<Value>,
    {
        reader().and_then(|config| {
            let clone_path = config["clone_path"].as_str().expect("field 'clone_path' is missing");
            let repositories = config.get("repositories").expect("field 'repositories' is missing");

            match Repositories::read_from(|| Ok(repositories.clone())) {
                Ok(repos) => Ok(Self {
                    clone_path: ClonePath(clone_path.to_string()),
                    repositories: repos,

                }),
                Err(err) => Err(err),
            }
        })
    }
}

impl BardoConfig {
   pub fn read_from<F>(reader: F) -> Result<Self>
    where
        F: Fn() -> Result<Value>,
    {
        reader().map(|toml| {
            let mut map: HashMap<String, Configuration> = HashMap::new();
            for (k, v) in toml.as_table().as_ref().expect("file has invalid format").iter() {
                let _ = Configuration::read_from(|| Ok(v.clone())).map(|creds| map.insert(k.to_string(), creds));
            }

            Self {
                profiles: map,
            }
        })
    }

    pub fn write_to<F>(&self, writer: F) -> Result<()>
    where
        F: Fn(&BardoConfig) -> Result<()>,
    {
        writer(&self)
    }
}

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

#[cfg(test)]
mod config_tests {
    use super::*;
    use std::env;
    use crate::file::read_bytes;
    // use std::process::Command;

    // #[test]
    // fn test_command() {
    //     let clone_repo = |path: &str, repo: &Value| {
    //         let org = repo.get("org").unwrap();
    //         let name = repo.get("name").unwrap();
    //         let ssh_url = format!("git@github.com:{}/{}.git",
    //                                  org.as_str().unwrap(),
    //                                  name.as_str().unwrap());

    //         let status = Command::new("sh")
    //             .current_dir(path)
    //             .arg("-c")
    //             .arg(format!("git clone {}", ssh_url))
    //             .status()
    //             .expect("failed to execute process");

    //         println!("process exited with: {}", status);
    //     };

    //     consume_repos2(clone_repo);
    // }

    #[test]
    fn test_configuration() {
        let toml_str = r#"
            clone_path = "/path"
            repositories = [
              {org = "crvshlab", name="repo1"},
              {org = "crvshlab", name="repo2"},
              {org = "crvshlab", regex="node-*"},
            ]
        "#;

        let reader = || read_bytes(toml_str.as_bytes());
        let config = Configuration::read_from(reader).expect("invalid format");
        assert_eq!("/path".to_string(), config.clone_path().0);
        assert_eq!(false, config.repositories.0.is_empty());
        assert_eq!("crvshlab".to_string(), config.repositories.0[0].org.0);
        assert_eq!("repo1".to_string(), config.repositories.0[0].name().unwrap().0);
    }

    #[test]
    fn test_bad_configuration_missing_opts() {
        let toml_str = r#"
            clone_path = "/path"
            repositories = [
              {org = "crvshlab"},
            ]
        "#;

        let reader = || read_bytes(toml_str.as_bytes());
        let config = Configuration::read_from(reader);
        assert_eq!(true, config.is_err());
    }

    #[test]
    fn test_bad_configuration_both_opts() {
        let toml_str = r#"
            clone_path = "/path"
            repositories = [
              {org = "crvshlab", name = "abc", regex = "a*"},
            ]
        "#;

        let reader = || read_bytes(toml_str.as_bytes());
        let config = Configuration::read_from(reader);
        assert_eq!(true, config.is_err());
    }

    #[test]
    fn test_add_repo_to_configuration() {
        let toml_str = r#"
            clone_path = "/path"
            repositories = [
              {org = "crvshlab", name="repo1"},
              {org = "crvshlab", name="repo2"},
              {org = "crvshlab", regex="node-*"},
            ]
        "#;


        let reader = || read_bytes(toml_str.as_bytes());
        let config = &mut Configuration::read_from(reader).expect("");
        let repos = config.repositories_mut();
        repos.add(Repository::new(
            Org("test".to_string()),
            Some(Name("foo".to_string())),
            None,
        ));

        assert_eq!(4, repos.0.len());
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

//     #[test]
//     fn test_print_ssh_urls() {
//         let print_url = |repo: &Value| {
//             let org = repo.get("org").unwrap();
//             let name = repo.get("name").unwrap();
//             let url = format!("git@github.com:{}/{}.git",
//                                      org.as_str().unwrap(),
//                                      name.as_str().unwrap());
//             assert_eq!(url, "git@github.com:crvshlab/backoffice-is-analysis.git");
//         };

//         consume_repos(print_url);
//     }

//     #[test]
//     fn test_repository_file() {
//         assert_eq!(config_file(), Some(PathBuf::from("/Users/seka/.config/bardo/gh/config")));

//         env::set_var("BARDO_CONFIG_HOME", "/Users/seka/foobar");
//         assert_eq!(config_file(), Some(PathBuf::from("/Users/seka/foobar/gh/config")));
//         env::remove_var("BARDO_CONFIG_HOME");
//     }

//     #[test]
//     fn test_config() {
//         let toml_str = r#"[default]
// clone_path = "/Users/seka/projects/mttrbit/bardo-repos"
// repositories = [
//   {org = "crvshlab", name = "test"}
// , {org = "crvshlab", regex = "nodejs-*"}
// ]
// "#;

//         let decoded: Value = toml::from_str(toml_str).unwrap();
//         let _profile = &decoded["default"];
//         // println!("repo file: {:?}", profile["repositories"]);
//     }

//     #[test]
//     fn test_config_dir() {
//         config_dir().map(|buf| {
//             assert_eq!(buf.as_path().to_str(), Some("/Users/seka/.config/bardo/gh"))
//         });
//     }
}
