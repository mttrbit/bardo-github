use client::client::{Github, Executor};
use config::context::BardoContext;

pub struct Command {
    context: BardoContext,
    gh: Github,
}

impl Command {
    pub fn new(ctx: BardoContext, gh: Github) -> Self {
        Self {
            context: ctx,
            gh: gh,
        }
    }

    pub fn run(&self) {
        let res = self.gh
            .get()
            .user()
            .emails()
            .execute::<serde_json::Value>()
            //.execute::<Vec<User>>()
            .unwrap();
        println!("response {:#?}", res);
    }
}
