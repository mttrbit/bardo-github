use crate::cmd::{Command, HttpResponse};
use client::client::{Github, Result,Executor};

#[derive(Deserialize, Debug)]
pub struct Repository {
    full_name: String,
    has_projects: bool,
    has_wiki: bool,
    open_issues_count: u32,
}

impl Repository {
    pub fn full_name(&self) -> &String {
        &self.full_name
    }

    pub fn has_projects(&self) -> &bool {
        &self.has_projects
    }

    pub fn has_wiki(&self) -> &bool {
        &self.has_wiki
    }

    pub fn open_issue_count(&self) -> &u32 {
        &self.open_issues_count
    }
}

pub struct GetRepoCmd<'a>(pub &'a Github, pub &'a str, pub &'a str);

impl<'a> Command<HttpResponse<Repository>> for GetRepoCmd<'a> {
    fn execute(&self) -> Result<HttpResponse<Repository>> {
        let result = self
            .0
            .get()
            .repos()
            .owner(self.1)
            .repo(self.2)
            .execute::<Repository>();

        result
    }
}

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

impl<'a> Command<HttpResponse<Sha>> for GetLatestCommitCmd<'a> {
    fn execute(&self) -> Result<HttpResponse<Sha>> {
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
