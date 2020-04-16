use crate::cmd::{Command, IterableCommand, HttpResponse, ResultIterator, PrintStd};
use crate::cmd::CommandExecutor;
use crate::commands::repo::get::{FetchRepoCmd, Repository};
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

pub struct GetIssuesCommand {
    context: BardoContext,
    gh: Github,
}

impl GetIssuesCommand {
    pub fn new(ctx: BardoContext, gh: Github) -> Self {
        Self {
            context: ctx,
            gh: gh,
        }
    }

    fn run(&self, org: &str, name: &str, b_print_all: bool) {
        let cmd: FetchOpenIssuesCmd = FetchOpenIssuesCmd::new(&self.gh, org, name);
        let (_, _, repo_res) = FetchRepoCmd(&self.gh, org, name).execute().unwrap();
        let repo: Repository = repo_res.unwrap();
        let full_name = repo.full_name();
        let num_total_issues = *repo.open_issue_count();
        let mut issues_mut: Vec<Issue>;
        println!("");

        let mut iter = cmd.execute_iter().into_iter();

        if b_print_all == false {
            let (_, _, res) = iter.next().unwrap().unwrap();
            issues_mut = res.unwrap();
            let num_fetched_issues = issues_mut.len();
            println!(
                "Showing {} of {} open issues in {}",
                num_fetched_issues, num_total_issues, full_name
            );
        } else {
            issues_mut = Vec::with_capacity(num_total_issues.try_into().unwrap());
            for next in iter {
                let (_, _, res) = next.unwrap();
                issues_mut.append(res.unwrap().as_mut());
            }

            println!(
                "Showing {} open issues in {}",
                num_total_issues, full_name
            );
        }

        println!("");

        issues_mut.to_std_out();
    }
}

impl<'a> CommandExecutor for GetIssuesCommand {

    fn execute(&self, args: &Vec<Vec<&str>>) {
        let profile = self.context.profile();
        let section = &self.context.config().get_profiles()[profile];
        let repositories = section.repositories();
        let print_all = crate::utils::pick_all(args);
        let maybe_repo = crate::utils::pick_repo(args);

        repositories
            .iter()
            .filter(|r| crate::utils::maybe_filter_repo(r, &maybe_repo))
            .for_each(|repo| match (repo.org(), repo.name()) {
                (o, Some(n)) => self.run(&o.0, &n.0, print_all),
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
