use crate::cmd::{Command, HttpResponse};
use client::client::{Github, Result, Executor};

pub struct UpdateFileCmd<'a>(pub &'a Github, pub &'a str, pub &'a str, pub &'a str, pub &'a serde_json::Value);

impl<'a> Command<HttpResponse<serde_json::Value>> for UpdateFileCmd<'a> {
    fn execute(&self) -> Result<HttpResponse<serde_json::Value>> {
        let result = self
            .0
            .put(self.4)
            .repos()
            .owner(self.1)
            .repo(self.2)
            .contents()
            .path(self.3)
            .execute::<serde_json::Value>();

        result
    }
}
