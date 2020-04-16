use crate::cmd::{Command, HttpResponse};
use client::client::{Github, Result, Executor};
use config::context::BardoContext;
use crate::commands::repo::post::CreateBranchCmd;
use crate::commands::repo::get::{GetLatestCommitCmd, Sha};

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
        let maybe_repo = crate::utils::pick_repo(args);
        let profile = self.context.profile();
        let section = &self.context.config().get_profiles()[profile];
        let repositories = section.repositories();
        let path = &section.clone_path().0;
        let mut cmd = "";
        let mut branch = "";
        for v in args.iter() {
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

        let temp_clone_path = format!("{}/.temp", path);
        repositories
            .iter()
            .filter(|r| crate::utils::maybe_filter_repo(r, &maybe_repo))
            .for_each(|repo| match (repo.org(), repo.name()) {
                (o, Some(n)) => {
                    // let _ = crate::commands::repo::clone::CloneRepoCommand::new(&temp_clone_path, &o.0, &n.0).execute();
                    let project_path = [&temp_clone_path, "/", &n.0].concat();
                    self.apply(&project_path, &o.0, &n.0, cmd, branch)
                },
                (_, _) => (),
            });
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
            let files = ListChangedFilesCommand::new(path).execute().unwrap();
            if !files.is_empty() {
                let (_, _, res) = self.get_latest_commit(org, name).unwrap();
                let sha = res.unwrap();
                self.create_branch(org, name, branch, sha.sha());
                // println!("file: {:?}", GetFileCommand(&self.gh, org, name, &files[0]).execute().unwrap().2.unwrap().sha());
            }
        }
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

struct ListChangedFilesCommand<'a> {
    path: &'a str,
}

impl<'a> ListChangedFilesCommand<'a> {
    pub fn new(path: &'a str) -> Self {
        Self {
            path: path,
        }
    }
}

/// Create a vector of changed files.
///
/// This function looks for modified or untracked files using `git status`.
impl<'a> Command<Vec<String>> for ListChangedFilesCommand<'a> {
    fn execute(&self) -> Result<Vec<String>> {
        let changes = std::process::Command::new("sh")
            .current_dir(self.path)
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

        Ok(files)
    }
}

struct GetFileCommand<'a>(pub &'a Github, pub &'a str, pub &'a str, pub &'a str);

impl<'a> Command<HttpResponse<Sha>> for GetFileCommand<'a> {
    fn execute(&self) -> Result<HttpResponse<Sha>> {
        let result = self
            .0
            .get()
            .repos()
            .owner(self.1)
            .repo(self.2)
            .contents()
            .path(self.3)
            .execute::<Sha>();

        result
    }
}
