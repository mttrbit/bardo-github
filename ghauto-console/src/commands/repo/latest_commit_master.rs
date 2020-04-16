use crate::cmd::Command;
use client::client::{Github, Result,Executor};
use reqwest::header::HeaderMap;
use reqwest::StatusCode;
use reqwest::header::{HeaderValue, ACCEPT};

#[derive(Deserialize, Debug)]
pub struct Sha {
    sha: String,
}

impl Sha {
    pub fn sha(&self) -> &String {
        &self.sha
    }
}

pub struct GetLatestCommitCmd<'a>(pub &'a Github, pub &'a str, pub &'a str, pub &'a str);

impl<'a> Command<Sha> for GetLatestCommitCmd<'a> {
    fn execute(&self) -> Result<(HeaderMap, StatusCode, Option<Sha>)> {
        let result = self
            .0
            .get()
            //.set_header(ACCEPT, HeaderValue::from_static("application/vnd.github.VERSION.sha"))
            .repos()
            .owner(self.1)
            .repo(self.2)
            .commits()
            .reference(self.3)
            .execute::<Sha>();

        result
    }
}
