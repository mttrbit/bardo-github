use crate::cmd::{Command, IterableCommand, HttpResponse, ResultIterator, PrintStd};
use crate::cmd::CommandExecutor;
use crate::commands::repo::get::{GetRepoCmd, Repository};
use crate::display::FmtDuration;
use client::client::{Executor, Github, Result};
use config::context::BardoContext;

use chrono::{DateTime, Duration, Utc};
use itertools::Itertools;
use prettytable::{format, Table};
use std::convert::TryInto;
use std::fmt::{Display, Formatter, Result as FmtResult};
use termion::{color, style};

#[derive(Deserialize, Debug)]
pub struct IssueLabel {
    name: String,
    color: String,
}

#[derive(Deserialize, Debug)]
pub struct Issue {
    number: i32,
    title: String,
    labels: Vec<IssueLabel>,
    repository_url: String,
    updated_at: String,
}

pub struct GetIssuesCommand<'a> {
    gh: &'a Github,
    org: &'a str,
    name: &'a str,
    b_print_all: bool,
}

pub struct GetIssuesCommandResult(pub String, pub Vec<Issue>, pub u32, pub Option<u32>);

impl<'a> PrintStd for GetIssuesCommandResult {
    fn to_std_out(&self) {
        let full_name = &self.0;
        let issues = &self.1;
        let total_issues = &self.2;
        let maybe_fetched_issues = &self.3;

        println!("");
        if maybe_fetched_issues.is_some() {
            println!(
                "Showing {} of {} open issues in {}",
                maybe_fetched_issues.unwrap(), total_issues, full_name
            );
        } else {
            println!(
                "Showing {} open issues in {}",
                total_issues, full_name
            );
        }
        println!("");

        issues.to_std_out();
    }
}

impl<'a> GetIssuesCommand<'a> {
   pub fn new(gh: &'a Github, org: &'a str, name: &'a str, b_print_all: bool) -> Self {
        Self {
            gh: gh,
            org: org,
            name: name,
            b_print_all: b_print_all,
        }
    }
}

impl<'a> Command<GetIssuesCommandResult> for GetIssuesCommand<'a> {
    fn execute(&self) -> Result<GetIssuesCommandResult> {
        let cmd: FetchOpenIssuesCmd = FetchOpenIssuesCmd::new(&self.gh, self.org, self.name);
        let (_, _, repo_res) = GetRepoCmd(&self.gh, self.org, self.name).execute().unwrap();
        let repo: Repository = repo_res.unwrap();
        let full_name = repo.full_name();
        let num_total_issues = *repo.open_issue_count();
        println!("");

        let mut iter = cmd.execute_iter().into_iter();

        if self.b_print_all == false {
            let (_, _, res) = iter.next().unwrap().unwrap();
            let issues = res.unwrap();
            let fetched_issues = issues.len().try_into().unwrap();
            return Ok(GetIssuesCommandResult(full_name.to_string(), issues, num_total_issues, Some(fetched_issues)));
        } else {
            let mut issues_mut = Vec::with_capacity(num_total_issues.try_into().unwrap());
            for next in iter {
                let (_, _, res) = next.unwrap();
                issues_mut.append(res.unwrap().as_mut());
            }

            return Ok(GetIssuesCommandResult(full_name.to_string(), issues_mut, num_total_issues, None));
        }
    }
}

pub struct GetIssuesCommandExecutor {
    gh: Github,
    context: BardoContext,
}

impl GetIssuesCommandExecutor {
     pub fn new(gh: Github, context: BardoContext) -> Self {
        Self {
            gh: gh,
            context: context,
        }
    }
}

impl<'a> CommandExecutor for GetIssuesCommandExecutor {

    fn execute(&self, args: &Vec<Vec<&str>>) {
        let profile = self.context.profile();
        let section = &self.context.config().get_profiles()[profile];
        let repositories = section.repositories();
        let print_all = crate::utils::print_all(args);
        let maybe_repo = crate::utils::pick_repo(args);

        repositories
            .iter()
            .filter(|r| crate::utils::maybe_filter_repo(r, &maybe_repo))
            .for_each(|repo| match (repo.org(), repo.name()) {
                (o, Some(n)) => {match GetIssuesCommand::new(&self.gh, &o.0, &n.0, print_all).execute() {
                    Ok(res) => res.to_std_out(),
                    Err(_) => (),
                }},
                (_, _) => (),
            });
    }
}

struct Issues<'a>(&'a Vec<Issue>);

impl<'a> PrintStd for Vec<Issue> {
    fn to_std_out(&self) {
        Issues(self).to_std_out();
    }
}

struct Label<'a>(&'a Vec<IssueLabel>);

impl<'a> Display for Label<'a> {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        let v = self.0;
        if v.is_empty() {
            return write!(f, "");
        } else {
            let ls = v.iter().take(3).map(|i| &i.name).join(", ");
            if v.len() > 3 {
                return write!(f, "({}, ...)", ls);
            } else {
                return write!(f, "({})", ls);
            }
        }
    }
}

impl<'a> PrintStd for Issues<'a> {
    fn to_std_out(&self) {
        let v = self.0;
        let mut table = Table::new();
        let format = format::FormatBuilder::new().padding(1, 1).build();
        table.set_format(format);
        table.set_titles(row![
            format!(
                "{}{}id{}",
                style::Bold,
                color::Fg(color::Green),
                style::Reset
            ),
            format!(
                "{}{}title{}",
                style::Bold,
                color::Fg(color::Magenta),
                style::Reset
            ),
            format!(
                "{}{}labels{}",
                style::Bold,
                color::Fg(color::White),
                style::Reset
            ),
            format!(
                "{}{}last update{}",
                style::Bold,
                color::Fg(color::White),
                style::Reset
            ),
        ]);

        let now = Utc::now();
        for e in v.iter() {
            let labels = Label(&e.labels);
            let dt_8601 = DateTime::parse_from_rfc3339(&e.updated_at).unwrap();
            let ago: Duration = now.signed_duration_since(dt_8601);
            table.add_row(row![
                format!("{}#{}{}", color::Fg(color::Green), e.number, style::Reset),
                format!("{}{}{}", color::Fg(color::Magenta), e.title, style::Reset),
                format!("{}{}{}", color::Fg(color::White), labels, style::Reset),
                format!(
                    "{}{}{}",
                    color::Fg(color::White),
                    FmtDuration::fuzzy_ago(ago),
                    style::Reset
                ),
            ]);
        }
        table.printstd();
    }
}

pub struct FetchOpenIssuesCmd<'a> {
    gh: &'a Github,
    owner: &'a str,
    name: &'a str,
}

impl<'a> FetchOpenIssuesCmd<'a> {
    fn new(gh: &'a Github, owner: &'a str, name: &'a str) -> Self {
        Self {
            gh: gh,
            owner: owner,
            name: name,
        }
    }
}

impl<'a> IterableCommand<Vec<Issue>> for FetchOpenIssuesCmd<'a> {
    fn execute_iter(&self) -> ResultIterator<Vec<Issue>> {
        fn call<'a>(
            gh: &'a Github,
            owner: &'a str,
            name: &'a str,
        ) -> Box<dyn Fn(&str) -> Result<HttpResponse<Vec<Issue>>> + 'a> {
            Box::new(move |page| {
                gh.get()
                    .repos()
                    .owner(owner)
                    .repo(name)
                    .issues()
                    .page(page)
                    .execute::<Vec<Issue>>()
            })
        }

        let t = call(self.gh, self.owner, self.name);
        ResultIterator::new(t, Some("1".to_string()))
    }
}
