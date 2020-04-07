use crate::cmd::{Command, HttpResponse};
use client::client::Github;
use config::context::BardoContext;

pub struct CloneRepoCommand {
    context: BardoContext,
    gh: Github,
}

impl CloneRepoCommand {
    pub fn new(ctx: BardoContext, gh: Github) -> Self {
        Self {
            context: ctx,
            gh: gh,
        }
    }

    pub fn run(&self, args: &Vec<Vec<&str>>) {
        let mut print_single_repo = false;
        let mut org = "";
        let mut name = "";
        for v in args.iter() {
            if v.contains(&"REPO") {
                print_single_repo = true;
                let mut split: std::str::Split<&str> = v[1].split("/");
                org = split.next().expect("organisation missing");
                name = split.next().expect("name missing");
            }
        }

        let profile = self.context.profile();
        let path = &self.context.config().get_profiles()[profile].clone_path().0;
        println!("");
        println!("start cloning repos in {}", path);
        println!("");
        if print_single_repo {
            self.clone(&path, org, name);
        } else {
            let repositories = self.context.config().get_profiles()[profile].repositories();
            for r in repositories.iter() {
                match (r.org(), r.name()) {
                    (o, Some(n)) => self.clone(&path, &o.0, &n.0),
                    (_, _) => (),
                };
            }
        }
    }

    fn clone(&self, path: &str, org: &str, name: &str) {
        let ssh_url = format!("git@github.com:{}/{}.git", org, name);

        let status = std::process::Command::new("sh")
            .current_dir(path)
            .arg("-c")
            .arg(format!("git clone {}", ssh_url))
            .status()
            .expect("failed to execute process");

        println!("process exited with: {}", status);
    }
}
