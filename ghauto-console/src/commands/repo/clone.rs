use crate::cmd::Command;
use crate::cmd::CommandExecutor;
use client::client::Result;
use config::context::BardoContext;

pub struct CloneRepoCommand<'a> {
    path: &'a str,
    org: &'a str,
    name: &'a str,
}

impl<'a> CloneRepoCommand<'a> {
    pub fn new(path: &'a str, org: &'a str, name: &'a str) -> Self {
        Self {
            path: path,
            org: org,
            name: name,
        }
    }
}

impl<'a> Command<std::process::ExitStatus> for CloneRepoCommand<'a> {
    fn execute(&self) -> Result<std::process::ExitStatus> {
        let ssh_url = format!("git@github.com:{}/{}.git", self.org, self.name);

        let status = std::process::Command::new("sh")
            .current_dir(self.path)
            .arg("-c")
            .arg(format!("git clone {}", ssh_url))
            .status()
            .expect("failed to execute process");

        println!("process exited with: {}", status);

        Ok(status)
    }
}

pub struct CloneRepoCommandExecutor {
    context: BardoContext,
}

impl CloneRepoCommandExecutor {
     pub fn new(ctx: BardoContext) -> Self {
        Self {
            context: ctx,
        }
    }
}

impl<'a> CommandExecutor for CloneRepoCommandExecutor {
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
                (o, Some(n)) => {let _ = CloneRepoCommand::new(&path, &o.0, &n.0).execute();},
                (_, _) => (),
            });
    }
}
