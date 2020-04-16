use crate::cmd::CommandExecutor;
use client::client::Github;
use config::config::Repository;
use config::context::BardoContext;

pub struct CloneRepoCommand {
    context: BardoContext,
}

impl CloneRepoCommand {
    pub fn new(ctx: BardoContext, gh: Github) -> Self {
        Self {
            context: ctx,
        }
    }

    fn run(&self, path: &str, org: &str, name: &str) {
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

impl<'a> CommandExecutor for CloneRepoCommand {
    fn execute(&self, args: &Vec<Vec<&str>>) {
        let maybe_repo = crate::utils::pick_repo(args);
        let profile = self.context.profile();
        let section = &self.context.config().get_profiles()[profile];
        let repositories = section.repositories();
        let path = &section.clone_path().0;

        println!("");
        println!("start cloning repos in {}", path);
        println!("");

        repositories
            .iter()
            .filter(|r| crate::utils::maybe_filter_repo(r, &maybe_repo))
            .for_each(|repo| match (repo.org(), repo.name()) {
                (o, Some(n)) => self.run(&path, &o.0, &n.0),
                (_, _) => (),
            });
    }
}
