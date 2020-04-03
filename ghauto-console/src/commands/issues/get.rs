use client::client::{Executor, Github, Result};
use config::context::BardoContext;
use crate::display::FmtDuration;

use chrono::{DateTime, Duration, Utc};
use itertools::Itertools;
use prettytable::{format, Table};
use reqwest::header::HeaderMap;
use reqwest::StatusCode;
use std::fmt::{Display, Formatter, Result as FmtResult};
use termion::{color, style};
use std::collections::HashMap;

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
pub struct Repo {
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

    fn fetch_repo_data(
        &self,
        owner: &str,
        repo: &str,
    ) -> Result<(HeaderMap, StatusCode, Option<Repo>)> {
        self.gh
            .get()
            .repos()
            .owner(owner)
            .repo(repo)
            // .execute::<serde_json::Value>()
            .execute::<Repo>()
    }

    fn fetch_open_issues(
        &self,
        owner: &str,
        repo: &str,
    ) -> Result<(HeaderMap, StatusCode, Option<Vec<Issue>>)> {
        self.gh
            .get()
            .repos()
            .owner(owner)
            .repo(repo)
            .issues()
            // .execute::<serde_json::Value>()
            .execute::<Vec<Issue>>()
    }

    fn fetch_open_issues_with_pages(
        &self,
        owner: &str,
        repo: &str,
        page: &str,
    ) -> Result<(HeaderMap, StatusCode, Option<Vec<Issue>>)> {
        self.gh
            .get()
            .repos()
            .owner(owner)
            .repo(repo)
            .issues()
            .page(page)
            .execute::<Vec<Issue>>()
    }

    pub fn run(&self, args: &Vec<&str>) {
        let (headers, _, res) = self
            .fetch_open_issues("crvshlab", "ciot-backoffice")
            .unwrap();
        let (_, _, repo_res) = self.fetch_repo_data("crvshlab", "ciot-backoffice").unwrap();
        let mut h = headers;
        let repo = repo_res.unwrap();
        let mut issues_mut: Vec<Issue> = res.unwrap();
        let num_fetched_issues = issues_mut.len();
        let num_total_issues = repo.open_issues_count;

        // println!("repos {:#?}", self.context.config().get_profiles()["default"].repositories());
        let print_all = args.contains(&"all");
        println!("");
        if print_all == false {
            println!(
                "Showing {} of {} open issues in {}",
                num_fetched_issues, num_total_issues, repo.full_name
            );
        } else {
            fn get_key_next(
                links: client::headers::Links,
            ) -> Option<HashMap<String, String>> {
                links.get("next").cloned()
            }

            fn get_key_page(next: HashMap<String, String>) -> Option<String> {
                next.get("page").cloned()
            }
            loop {
                let page = client::headers::link(&h)
                    .and_then(get_key_next)
                    .and_then(get_key_page);
                if page.is_some() {
                    let (headers, _, res) = self
                        .fetch_open_issues_with_pages("crvshlab", "ciot-backoffice", &page.unwrap())
                        .unwrap();
                    issues_mut.append(res.unwrap().as_mut());
                    h = headers;
                } else {
                    break;
                }
            }
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
