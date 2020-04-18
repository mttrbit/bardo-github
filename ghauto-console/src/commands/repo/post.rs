use crate::cmd::{Command, HttpResponse};
use client::client::{Github, Result,Executor};

pub struct CreateBranchCmd<'a>(pub &'a Github, pub &'a str, pub &'a str, pub &'a serde_json::Value);

#[derive(Deserialize, Debug)]
pub struct CreateBranchResponse {
    #[serde(rename(deserialize = "ref"))]
    reference: String,
    url: String,
}

impl CreateBranchResponse {
    pub fn reference(&self) -> &String {
        &self.reference
    }
}

impl<'a> Command<HttpResponse<CreateBranchResponse>> for CreateBranchCmd<'a> {
    fn execute(&self) -> Result<HttpResponse<CreateBranchResponse>> {
        let result = self
            .0
            .post(self.3)
            .repos()
            .owner(self.1)
            .repo(self.2)
            .git()
            .refs()
            .execute::<CreateBranchResponse>();

        result
    }
}

pub struct CreatePrCommand<'a>(pub &'a Github, pub &'a str, pub &'a str, pub &'a serde_json::Value);

#[derive(Deserialize, Debug)]
pub struct CreatePrResponse {
    number: i32,
    url: String,
}

impl CreatePrResponse {
    pub fn number(&self) -> &i32 {
        &self.number
    }
}

impl<'a> Command<HttpResponse<CreatePrResponse>> for CreatePrCommand<'a> {
    fn execute(&self) -> Result<HttpResponse<CreatePrResponse>> {
        let result = self
            .0
            .post(self.3)
            .repos()
            .owner(self.1)
            .repo(self.2)
            .pulls()
            .execute::<CreatePrResponse>();

        result
    }
}

pub struct UpdatePrCommand<'a>(pub &'a Github, pub &'a str, pub &'a str, pub &'a i32, pub &'a serde_json::Value);

impl<'a> Command<HttpResponse<serde_json::Value>> for UpdatePrCommand<'a> {
    fn execute(&self) -> Result<HttpResponse<serde_json::Value>> {
        let result = self
            .0
            .post(self.4)
            .repos()
            .owner(self.1)
            .repo(self.2)
            .issues(&format!("{}", self.3))
            .assignees()
            .execute::<serde_json::Value>();

        result
    }
}
