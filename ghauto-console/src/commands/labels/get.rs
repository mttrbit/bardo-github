use client::client::{Executor, Github};
use config::context::BardoContext;

pub struct GetLabelsCommand {
    _context: BardoContext,
    gh: Github,
}

impl GetLabelsCommand {
    pub fn new(ctx: BardoContext, gh: Github) -> Self {
        Self {
            _context: ctx,
            gh: gh,
        }
    }

    pub fn run(&self) {
        let res = self
            .gh
            .get()
            .repos()
            .owner("crvshlab")
            .repo("ciot-backoffice")
            .labels()
            .execute::<toml::Value>()
            .unwrap();

        println!("{:#?}", res);
    }
}
