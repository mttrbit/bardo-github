use crate::cmd::{Command, FetchAll, HttpResponse};
use crate::display::FmtDuration;
use client::client::{Executor, Github, Result};
use config::context::BardoContext;

use chrono::{DateTime, Duration, Utc};
use itertools::Itertools;
use prettytable::{format, Table};
use reqwest::header::HeaderMap;
use reqwest::StatusCode;
use std::fmt::{Display, Formatter, Result as FmtResult};
use termion::{color, style};
use std::convert::TryInto;

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

#[derive(Deserialize, Debug)]
pub struct Repository {
    full_name: String,
    has_projects: bool,
    has_wiki: bool,
    open_issues_count: u32,
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

    pub fn run(&self, args: &Vec<Vec<&str>>) {
        let mut print_all = false;
        let mut print_single_repo = false;
        let mut org = "";
        let mut name = "";
        for v in args.iter() {
            if v.contains(&"ALL") {
                print_all = true;
            }
            if v.contains(&"REPO") {
                print_single_repo = true;
                let mut split: std::str::Split<&str> = v[1].split("/");
                org = split.next().expect("organisation missing");
                name = split.next().expect("name missing");
            }
        }
        if print_single_repo {
            self.fetch_issues(org, name, print_all);
        } else {
            let repositories = self.context.config().get_profiles()["default"].repositories();
            for r in repositories.iter() {
                match (r.org(), r.name()) {
                    (o, Some(n)) => self.fetch_issues(&o.0, &n.0, print_all),
                    (_, _) => (),
                };
            }
        }
    }

    fn fetch_issues(&self, org: &str, name: &str, b_print_all: bool) {
        let cmd: FetchOpenIssuesCmd = FetchOpenIssuesCmd(&self.gh, org, name);

        let (_, _, repo_res) = FetchRepoCmd(&self.gh, org, name).execute().unwrap();
        let repo: Repository = repo_res.unwrap();
        let num_total_issues = repo.open_issues_count;
        let mut issues_mut: Vec<Issue>;
        println!("");
        if b_print_all == false {
            let (_, _, res) = cmd.execute().unwrap();
            issues_mut = res.unwrap();
            let num_fetched_issues = issues_mut.len();
            println!(
                "Showing {} of {} open issues in {}",
                num_fetched_issues, num_total_issues, repo.full_name
            );
        } else {
            issues_mut = Vec::with_capacity(num_total_issues.try_into().unwrap());
            cmd.fetch_all(issues_mut.as_mut());
            println!(
                "Showing {} open issues in {}",
                num_total_issues, repo.full_name
            );
        }

        println!("");

        issues_mut.to_std_out();
    }
}

struct Issues<'a>(&'a Vec<Issue>);

trait PrintStd {
    fn to_std_out(&self);
}

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

pub struct FetchRepoCmd<'a>(pub &'a Github, pub &'a str, pub &'a str);

impl<'a> Command<Repository> for FetchRepoCmd<'a> {
    fn execute(&self) -> Result<(HeaderMap, StatusCode, Option<Repository>)> {
        let result = self
            .0
            .get()
            .repos()
            .owner(self.1)
            .repo(self.2)
            .execute::<Repository>();

        result
    }
}

pub struct FetchOpenIssuesCmd<'a>(pub &'a Github, pub &'a str, pub &'a str);

impl<'a> Command<Vec<Issue>> for FetchOpenIssuesCmd<'a> {
    fn execute(&self) -> Result<(HeaderMap, StatusCode, Option<Vec<Issue>>)> {
        let result = self
            .0
            .get()
            .repos()
            .owner(self.1)
            .repo(self.2)
            .issues()
            .execute::<Vec<Issue>>();

        result
    }
}

impl<'a> FetchAll<Vec<Issue>> for FetchOpenIssuesCmd<'a> {
    fn fetch_all(&self, issues_mut: & mut Vec<Issue>) {
        let mut page = Some("1".to_string());

        fn call(gh: &Github, owner: &str, name: &str, page: &str) -> Result<(HeaderMap, StatusCode, Option<Vec<Issue>>)> {
            gh
                .get()
                .repos()
                .owner(owner)
                .repo(name)
                .issues()
                .page(page)
                .execute::<Vec<Issue>>()
        };

        loop {
            if page.is_some() {
                let (headers, _, res) = call(self.0, self.1, self.2, &page.unwrap()).unwrap();
                issues_mut.append(res.unwrap().as_mut());
                page = self.read_page_from_link_header(&headers);
            } else {
                break;
            }
        }
    }
}
