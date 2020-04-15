use crate::cmd::{Command, HttpResponse};
use client::client::Github;
use config::context::BardoContext;

pub struct ApplyCommand {
    context: BardoContext,
    gh: Github,
}

// Iterates over all configured repositories, clones each of them into a temporary folder,
// and runs command to apply any changes you need. After command is run, git status is
// executed and all added and changed files are committed into a new branch branch with
// commit message message, and then a new pull request is created with comment comment
// and the given list of reviewers.
impl ApplyCommand {
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
        let mut cmd = "";
        for v in args.iter() {
            if v.contains(&"REPO") {
                print_single_repo = true;
                let mut split: std::str::Split<&str> = v[1].split("/");
                org = split.next().expect("organisation missing");
                name = split.next().expect("name missing");
            }

            if v.contains(&"CMD") {
                cmd = v[1]
            }

            if v.contains(&"BRANCH") {
                println!("branch {}", v[1]);
            }

            if v.contains(&"MESSAGE") {
                println!("message {}", v[1]);
            }

            if v.contains(&"COMMENT") {
                println!("comment {}", v[1]);
            }

            if v.contains(&"REVIEWERS") {
                println!("reviewers {}", v[1]);
            }
        }

        let profile = self.context.profile();
        let clone_path = &self.context.config().get_profiles()[profile].clone_path().0;

        if print_single_repo {
            let project_path = [clone_path, "/", name].concat();
            self.apply(&project_path, org, name, cmd);
        } else {
            let repositories = self.context.config().get_profiles()[profile].repositories();
            for r in repositories.iter() {
                match (r.org(), r.name()) {
                    (o, Some(n)) => {
                        let project_path = [clone_path, "/", &n.0].concat();
                        self.apply(&project_path, &o.0, &n.0, cmd)
                    }
                    (_, _) => (),
                };
            }
        }
    }

    fn apply(&self, path: &str, org: &str, name: &str, cmd: &str) {
        let ssh_url = format!("git@github.com:{}/{}.git", org, name);
        println!("");
        println!("");
        println!("applying the command {} to {}", cmd, path);
        let status = std::process::Command::new("sh")
            .current_dir(path)
            .arg("-c")
            .arg(cmd)
            .status()
            .expect("failed to execute process");
        if status.success() {
            println!("files {:?}", self.changed_files(path));
        }
    }

    fn clone_repos() {}

    /// Create a vector of changed files.
    ///
    /// This function looks for modified or untracked files using `git status`.
    fn changed_files(&self, path: &str) -> Vec<String> {
        let changes = std::process::Command::new("sh")
            .current_dir(path)
            .arg("-c")
            .arg("git status --porcelain --untracked-files")
            .output()
            .ok()
            .expect("failed to execute process");
        let a_vec = changes.stdout.to_owned();
        let sort: Vec<_> = a_vec.split(|i| *i == 10).filter(|line| !line.is_empty()).collect();
        let mut files: Vec<String> = Vec::with_capacity(sort.len());
        let re = regex::bytes::Regex::new(r"^(?: \x4d|\x3f\x3f) (.*)$").unwrap();
        for line in sort.iter() {
            match re.captures(line) {
                Some(m) => {
                    let file = std::str::from_utf8(&m[1]).unwrap();
                    files.push(file.to_owned());
                },
                _ => (),
            }
        }

        files
    }

    fn create_branch() {}

    fn commit_branch() {}

    fn create_pr() {}
}
