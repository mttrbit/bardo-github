use crate::cmd::CommandExecutor;
use client::client::Github;
use config::config::Repository;
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

fn pick_repo<'a>(args: &'a Vec<Vec<&'a str>>) -> Option<(&'a str, &'a str)> {
    for v in args {
        if v[0] == "REPO" {
            let mut split: std::str::Split<&str> = v[1].split("/");
            let org = split.next().expect("organisation missing");
            let name = split.next().expect("name missing");

            return Some((org, name));
        }
    }

    return None;
}

impl<'a> CommandExecutor for CloneRepoCommand {
    fn execute(&self, args: &Vec<Vec<&str>>) {
        let maybe_repo = pick_repo(args);
        let profile = self.context.profile();
        let section = &self.context.config().get_profiles()[profile];
        let path = &section.clone_path().0;
        let repositories = section.repositories();

        println!("");
        println!("start cloning repos in {}", path);
        println!("");

        repositories
            .iter()
            .filter(|r| self::maybe_filter_repo(r, &maybe_repo))
            .for_each(|repo| match (repo.org(), repo.name()) {
                (o, Some(n)) => self.run(&path, &o.0, &n.0),
                (_, _) => (),
            });
    }
}

fn maybe_filter_repo<'a>(
    repo: &'a Repository,
    arg: &'a Option<(&str, &str)>,
) -> bool {
    match Some((arg, repo.org(), repo.name())) {
        Some((Some((org, name)), r_org, Some(r_name))) => {
            r_org.0 == org.to_string() && r_name.0 == name.to_string()
        }
        _ => true,
    }
}
