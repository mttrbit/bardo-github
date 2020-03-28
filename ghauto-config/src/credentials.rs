use std::collections::HashMap;
use std::io;
use std::path::PathBuf;

use serde_derive::Deserialize;

use crate::file::config_dir;

pub use io::Result;
#[derive(Debug,Deserialize)]
pub struct ClientId(pub String);
#[derive(Debug,Deserialize)]
pub struct ClientSecret(pub String);
#[derive(Debug,Deserialize)]
pub struct AccessToken(pub String);

#[derive(Debug,Deserialize)]
pub struct Credentials {
    client_id: ClientId,
    client_secret: ClientSecret,
    access_token: Option<AccessToken>,
}

#[derive(Debug,Deserialize)]
pub struct BardoCredentials {
    profiles: HashMap<String, Credentials>,
}

impl Clone for Credentials {
    fn clone(&self) -> Self {
        Self {
            client_id: ClientId(self.client_id.0.clone()),
            client_secret: ClientSecret(self.client_secret.0.clone()),
            access_token: match &self.access_token {
                Some(o) => Some(AccessToken(o.0.clone())),
                _ => None,
            },
        }
    }
}

impl Credentials {
    fn new(client_id: ClientId, client_secret: ClientSecret, access_token: Option<AccessToken>) -> Self {
        Self {
            client_id: client_id,
            client_secret: client_secret,
            access_token: access_token,
        }
    }

    pub fn client_id(&self) -> &ClientId {
        &self.client_id
    }

    pub fn client_secret(&self) -> &ClientSecret {
        &self.client_secret
    }

    pub fn access_token(&self) -> Option<&AccessToken> {
        self.access_token.as_ref()
    }

    pub fn access_token_mut(&mut self) -> &mut Option<AccessToken> {
        &mut self.access_token
    }

    pub fn read_from<F>(reader: F) -> Result<Self>
    where
        F: Fn() -> Result<toml::Value>,
    {
        reader().map(|section| {
            let str_client_id = section["bardo_client_id"].as_str().expect("field 'bardo_client_id' is missing");
            let str_client_secret = section["bardo_client_secret"].as_str().expect("field 'bardo_client_secret' is missing");
            let client_id = ClientId(str_client_id.to_string());
            let client_secret = ClientSecret(str_client_secret.to_string());
            let access_token = section.get("bardo_access_token").map(|f| AccessToken(f.as_str().unwrap().to_string()));
            Self {
                client_id: client_id,
                client_secret: client_secret,
                access_token: access_token,
            }
        })
    }

    pub fn write_to<F>(&self, writer: F) -> Result<()>
    where
        F: Fn(&Credentials) -> Result<()>,
    {
        writer(&self)
    }
}

impl BardoCredentials {

    pub fn read_from<F>(reader: F) -> Result<Self>
    where
        F: Fn() -> Result<toml::Value>,
    {
        reader().map(|toml| {
            let mut map: HashMap<String, Credentials> = HashMap::new();
            for (k, v) in toml.as_table().as_ref().expect("file has invalid format").iter() {
                let _ = Credentials::read_from(|| Ok(v.clone())).map(|creds| map.insert(k.to_string(), creds));
            }
           
            Self {
                profiles: map,
            }
        })
    }

    pub fn write_to<F>(&self, writer: F) -> Result<()>
    where
        F: Fn(&BardoCredentials) -> Result<()>,
    {
        writer(&self)
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::profile::profile;
    use crate::file::read_bytes;

    #[test]
    fn read_from() {
        let toml_str = r#"
            [default]
            bardo_client_id = "client_id"
            bardo_client_secret = "client_secret"
            bardo_access_token = "access_token"
        "#;

        let reader = || read_bytes(toml_str.as_bytes()).map(|toml| {
            let p = profile().expect("profile is not set");
            toml[p].clone()
        });

        let creds = Credentials::read_from(reader).expect("credentials not parsed");
        assert_eq!("client_id".to_string(), creds.client_id().0);
        assert_eq!("client_secret".to_string(), creds.client_secret().0);
        assert_eq!(false, creds.access_token().is_none());
    }

    #[test]
    fn read_from_full() {
        let toml_str = r#"
            [default]
            bardo_client_id = "client_id"
            bardo_client_secret = "client_secret"
            bardo_access_token = "access_token"

            [foo]
            bardo_client_id = "client_id"
            bardo_client_secret = "client_secret"
        "#;

        let reader = || read_bytes(toml_str.as_bytes());

        let config = BardoCredentials::read_from(reader).expect("config not parsed");
        assert_eq!(false, config.profiles.is_empty());
        assert_eq!(2, config.profiles.keys().len());
        assert_eq!(true, config.profiles.get("default").is_some());
        assert_eq!(true, config.profiles.get("default").unwrap().access_token().is_some());
        assert_eq!(true, config.profiles.get("foo").is_some());
        assert_eq!(true, config.profiles.get("foo").unwrap().access_token().is_none());
    }

    #[test]
    fn write_to() {

        let mut map: HashMap<String, Credentials> = HashMap::new();
        map.insert(
            "default".to_string(),
            Credentials {
                client_id: ClientId("id".to_string()),
                client_secret: ClientSecret("secret".to_string()),
                access_token: None,
            }
        );
        let config = BardoCredentials {
            profiles: map,
        };

        config.write_to(|c| {
            let creds = c.profiles.get("default").unwrap();
            let str = format!("{}:{}:{:?}", creds.client_id().0, creds.client_secret().0, creds.access_token());
            assert_eq!("id:secret:None", str);
            Ok(())
        }).expect("write_to panicked");

    }

    #[test]
    fn write_to_update_access_token() {

        let mut map: HashMap<String, Credentials> = HashMap::new();
        map.insert(
            "default".to_string(),
            Credentials {
                client_id: ClientId("id".to_string()),
                client_secret: ClientSecret("secret".to_string()),
                access_token: None,
            }
        );
        let mut config = BardoCredentials {
            profiles: map,
        };

        let creds = config.profiles.get_mut("default").unwrap();

        *creds.access_token_mut() = Some(AccessToken("token".to_string()));
       
        config.write_to(|c| {
            let creds = c.profiles.get("default").unwrap();
            let str = format!("{}:{}:{}", creds.client_id().0, creds.client_secret().0, creds.access_token().unwrap().0);
            assert_eq!("id:secret:token", str);
            Ok(())
        }).expect("write_to panicked");
    }
}
