use crate::cmd::{Command, HttpResponse};
use client::client::{Github, Result};
use config::context::BardoContext;
use crate::commands::repo::create_branch::CreateBranchCmd;
use crate::commands::repo::latest_commit_master::{GetLatestCommitCmd, Sha};

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
        let mut branch = "";
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
                branch = v[1]
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
            self.apply(&project_path, org, name, cmd, branch);
        } else {
            let repositories = self.context.config().get_profiles()[profile].repositories();
            for r in repositories.iter() {
                match (r.org(), r.name()) {
                    (o, Some(n)) => {
                        let project_path = [clone_path, "/", &n.0].concat();
                        self.apply(&project_path, &o.0, &n.0, cmd, branch)
                    }
                    (_, _) => (),
                };
            }
        }
    }

    fn apply(&self, path: &str, org: &str, name: &str, cmd: &str, branch: &str) {
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
            // println!("files {:?}", self.changed_files(path));
            let (_, _, res) = self.get_latest_commit(org, name).unwrap();
            let sha = res.unwrap();
            self.create_branch(org, name, branch, sha.sha());
        }
    }

    fn clone_repo(path: &str, org: &str, name: &str) -> std::io::Result<std::process::ExitStatus> {
        let ssh_url = format!("git@github.com:{}/{}.git", org, name);
        std::process::Command::new("sh")
            .current_dir(path)
            .arg("-c")
            .arg(format!("git clone {}", ssh_url))
            .status()
    }

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

    fn get_latest_commit(&self,  org: &str, name: &str) -> Result<HttpResponse<Sha>> {
        GetLatestCommitCmd(&self.gh, org, name, "heads/master").execute()
    }

    fn create_branch(&self,  org: &str, name: &str, branch: &str, sha: &str) {
        let a_ref = format!("refs/heads/{}", branch);
        let body = serde_json::json!({"ref": a_ref, "sha": sha});
        println!("create branch: {:?}", CreateBranchCmd(&self.gh, org, name, &body).execute().unwrap());
    }

    fn commit_branch() {}

    fn create_pr() {}
}
