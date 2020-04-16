use crate::cmd::Command;
use client::client::{Github, Result,Executor};
use reqwest::header::HeaderMap;
use reqwest::StatusCode;


pub struct CreateBranchCmd<'a>(pub &'a Github, pub &'a str, pub &'a str, pub &'a serde_json::Value);

impl<'a> Command<serde_json::Value> for CreateBranchCmd<'a> {
    fn execute(&self) -> Result<(HeaderMap, StatusCode, Option<serde_json::Value>)> {
        let result = self
            .0
            .post(self.3)
            .repos()
            .owner(self.1)
            .repo(self.2)
            .git()
            .refs()
            .execute::<serde_json::Value>();

        result
    }
}
