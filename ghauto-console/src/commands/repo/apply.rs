use crate::cmd::CommandExecutor;
use crate::cmd::{Command, HttpResponse};
use crate::commands::repo::get::{GetLatestCommitCmd, Sha};
use crate::commands::repo::post::{CreateBranchCmd, CreateBranchResponse};
use crate::commands::repo::post::{CreatePrCommand, CreatePrResponse, UpdatePrCommand};
use client::client::{Executor, Github, Result};
use config::context::BardoContext;

pub struct ApplyCommand<'a> {
    gh: &'a Github,
    path: &'a str,
    org: &'a str,
    name: &'a str,
    cmd: &'a str,
    branch: &'a str,
    message: &'a str,
    comment: &'a str,
    reviewers: &'a Option<Vec<String>>,
}

impl<'a> ApplyCommand<'a> {
    pub fn new(
        gh: &'a Github,
        path: &'a str,
        org: &'a str,
        name: &'a str,
        cmd: &'a str,
        branch: &'a str,
        message: &'a str,
        comment: &'a str,
        reviewers: &'a Option<Vec<String>>,
    ) -> Self {
        Self {
            gh: gh,
            path: path,
            org: org,
            name: name,
            cmd: cmd,
            branch: branch,
            message: message,
            comment: comment,
            reviewers: reviewers,
        }
    }

    fn update_file_in_branch(&self, file: &str) -> Result<HttpResponse<serde_json::Value>> {
        let (_, _, maybe_file_sha) = GetFileCommand(&self.gh, &self.org, &self.name, file)
            .execute()
            .unwrap();
        let file_path = [&self.path, "/", file].concat();
        let file_content = crate::config::file::read(file_path).unwrap();
        let file_content_base64 = base64::encode(file_content);
        let body = if maybe_file_sha.is_some() {
            let file_sha = maybe_file_sha.unwrap();
            serde_json::json!({"content": file_content_base64, "sha": file_sha.sha(), "branch": self.branch, "message": self.message})
        } else {
            serde_json::json!({"content": file_content_base64, "branch": self.branch, "message": self.message})
        };
        crate::commands::repo::put::UpdateFileCmd(&self.gh, &self.org, &self.name, file, &body)
            .execute()
    }

    fn get_latest_commit(&self) -> Result<HttpResponse<Sha>> {
        GetLatestCommitCmd(&self.gh, &self.org, &self.name, "heads/master").execute()
    }

    fn create_branch(&self, sha: &str) -> Result<HttpResponse<CreateBranchResponse>> {
        let a_ref = format!("refs/heads/{}", self.branch);
        let body = serde_json::json!({"ref": a_ref, "sha": sha});
        CreateBranchCmd(self.gh, self.org, self.name, &body).execute()
    }

    fn create_pr(&self, head: &str) -> Result<HttpResponse<CreatePrResponse>> {
        let base = "refs/heads/master";
        let body = serde_json::json!({"head": head, "base": base, "title": self.branch, "body": self.comment});
        CreatePrCommand(&self.gh, &self.org, &self.name, &body).execute()
    }

    fn update_pr(&self, number: &i32) -> Result<HttpResponse<serde_json::Value>> {
        let body = serde_json::json!({"assigness": self.reviewers});
        UpdatePrCommand(&self.gh, &self.org, &self.name, number, &body).execute()
    }
}

// Iterates over all configured repositories, clones each of them into a temporary folder,
// and runs command to apply any changes you need. After command is run, git status is
// executed and all added and changed files are committed into a new branch branch with
// commit message message, and then a new pull request is created with comment comment
// and the given list of reviewers.
impl<'a> Command<()> for ApplyCommand<'a> {
    fn execute(&self) -> Result<()> {
        println!("");
        println!("");
        println!("applying the command {} to {}", self.cmd, self.path);
        let status = std::process::Command::new("sh")
            .current_dir(self.path)
            .arg("-c")
            .arg(self.cmd)
            .status()
            .expect("failed to execute process");
        if status.success() {
            let files = ListChangedFilesCommand(self.path).execute().unwrap();
            if !files.is_empty() {
                let (_, _, maybe_commit_sha) = self.get_latest_commit().unwrap();
                let (_, _, maybe_branch) =
                    self.create_branch(maybe_commit_sha.unwrap().sha()).unwrap();

                for file in files {
                    match self.update_file_in_branch(&file) {
                        Err(e) => {
                            println!("An error occurred while updating files in branch {:?}", e);
                        }
                        _ => (),
                    }
                }

                let branch_response = maybe_branch.unwrap();
                let (_, _, maybe_pr) = self.create_pr(branch_response.reference()).unwrap();

                let pr = maybe_pr.unwrap();
                let pr_number = pr.number();
                self.update_pr(pr_number);
            }
        }

        Ok(())
    }
}

struct ListChangedFilesCommand<'a>(pub &'a str);

/// Create a vector of changed files.
///
/// This function looks for modified or untracked files using `git status`.
impl<'a> Command<Vec<String>> for ListChangedFilesCommand<'a> {
    fn execute(&self) -> Result<Vec<String>> {
        let changes = std::process::Command::new("sh")
            .current_dir(self.0)
            .arg("-c")
            .arg("git status --porcelain --untracked-files")
            .output()
            .ok()
            .expect("failed to execute process");
        let a_vec = changes.stdout.to_owned();
        let sort: Vec<_> = a_vec
            .split(|i| *i == 10)
            .filter(|line| !line.is_empty())
            .collect();
        let mut files: Vec<String> = Vec::with_capacity(sort.len());
        let re = regex::bytes::Regex::new(r"^(?: \x4d|\x3f\x3f) (.*)$").unwrap();
        for line in sort.iter() {
            match re.captures(line) {
                Some(m) => {
                    let file = std::str::from_utf8(&m[1]).unwrap();
                    files.push(file.to_owned());
                }
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

pub struct ApplyCommandExecutor {
    gh: Github,
    context: BardoContext,
}

impl ApplyCommandExecutor {
    pub fn new(gh: Github, context: BardoContext) -> Self {
        Self {
            gh: gh,
            context: context,
        }
    }
}

impl<'a> CommandExecutor for ApplyCommandExecutor {
    fn execute(&self, args: &Vec<Vec<&str>>) {
        let maybe_repo = crate::utils::pick_repo(args);
        let profile = self.context.profile();
        let section = &self.context.config().get_profiles()[profile];
        let repositories = section.repositories();
        let path = &section.clone_path().0;
        let cmd = crate::utils::pick_command(args).unwrap();
        let branch = crate::utils::pick_branch(args).unwrap();
        let message = crate::utils::pick_message(args).unwrap();
        let comment = crate::utils::pick_comment(args).unwrap();
        let reviewers = crate::utils::pick_reviewers(args);

        let temp_clone_path = format!("{}/.temp", path);
        repositories
            .iter()
            .filter(|r| crate::utils::maybe_filter_repo(r, &maybe_repo))
            .for_each(|repo| match (repo.org(), repo.name()) {
                (o, Some(n)) => {
                    // let _ = crate::commands::repo::clone::CloneRepoCommand::new(&temp_clone_path, &o.0, &n.0).execute();
                    let project_path = [&temp_clone_path, "/", &n.0].concat();
                    ApplyCommand::new(
                        &self.gh,
                        &project_path,
                        &o.0,
                        &n.0,
                        cmd,
                        branch,
                        message,
                        comment,
                        &reviewers
                    ).execute();
                }
                (_, _) => (),
            });
    }
}
