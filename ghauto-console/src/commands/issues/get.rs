use client::client::{Executor, Github, Result};
use config::context::BardoContext;

use chrono::{DateTime, Duration, FixedOffset, Local, NaiveDateTime, TimeZone, Utc};
use itertools::Itertools;
use prettytable::{format, Cell, Row, Table};
use reqwest::header::HeaderMap;
use reqwest::StatusCode;
use std::str::FromStr;
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
pub struct Repo {
    full_name: String,
    has_projects: bool,
    has_wiki: bool,
    open_issues_count: u32,
}

struct IssuesPrinter<'a>(&'a Vec<Issue>);

trait PrintStd {
    fn to_std_out(&self);
}

impl<'a> PrintStd for Vec<Issue> {
    fn to_std_out(&self) {
        IssuesPrinter(self).to_std_out();
    }
}

impl<'a> IssuesPrinter<'a> {
    fn print_labels(v: &Vec<IssueLabel>) -> String {
        if v.is_empty() {
            "".to_string()
        } else {
            format!(
                "({}{})",
                v.iter().take(3).map(|i| &i.name).join(", "),
                if v.len() > 3 { ", ..." } else { "" }
            )
        }
    }

    fn format_duration(num: i64, unit: &str) -> String {
        [
            num.to_string().as_ref(),
            " ",
            unit,
            if num == 1 { "s" } else { "" },
        ]
        .concat()
    }

    fn fuzzy_ago(ago: Duration) -> String {
        if ago.num_seconds() < 60 {
            return "less than a minute ago".to_string();
        }
        if ago.num_minutes() < 60 {
            return IssuesPrinter::format_duration(ago.num_minutes(), "minute");
        }
        if ago.num_hours() < 24 {
            return IssuesPrinter::format_duration(ago.num_hours(), "hour");
        }
        if ago.num_hours() < 720 {
            return IssuesPrinter::format_duration(ago.num_hours() / 24, "day");
        }
        if ago.num_hours() < 262800 {
            return IssuesPrinter::format_duration(ago.num_hours() / 720, "month");
        }

        IssuesPrinter::format_duration(ago.num_hours() / 262800, "year")
    }

    fn print_ago(now: &DateTime<Utc>, updated_at: &DateTime<FixedOffset>) -> String {
        let ago: Duration = now.signed_duration_since(*updated_at);
        format!("about {} ago", IssuesPrinter::fuzzy_ago(ago))
    }
}

impl<'a> PrintStd for IssuesPrinter<'a> {
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
            let labels = IssuesPrinter::print_labels(&e.labels);
            let dt_8601 = DateTime::parse_from_rfc3339(&e.updated_at).unwrap();
            let updated_at = IssuesPrinter::print_ago(&now, &dt_8601);
            table.add_row(row![
                format!("{}#{}{}", color::Fg(color::Green), e.number, style::Reset),
                format!("{}{}{}", color::Fg(color::Magenta), e.title, style::Reset),
                format!("{}{}{}", color::Fg(color::White), labels, style::Reset),
                format!("{}{}{}", color::Fg(color::White), updated_at, style::Reset),
            ]);
        }
        table.printstd();
    }
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
            // .execute::<serde_json::Value>()
            .execute::<Vec<Issue>>()
    }
    pub fn run(&self) {
        let (headers, status_code, res) = self
            .fetch_open_issues("crvshlab", "ciot-backoffice")
            .unwrap();
        let (_, _, repo_res) = self.fetch_repo_data("crvshlab", "ciot-backoffice").unwrap();
        let mut h = headers;
        let repo = repo_res.unwrap();
        let mut issues_mut: Vec<Issue> = res.unwrap();
        let num_fetched_issues = issues_mut.len();
        let num_total_issues = repo.open_issues_count;

        let print_all = false;
        println!("");
        if print_all == false {
            println!(
                "Showing {} of {} open issues in {}",
                num_fetched_issues, num_total_issues, repo.full_name
            );
        } else {
            fn get_key_next(links: client::headers::Links) -> Option<std::collections::HashMap<String, String>> {
                links.get("next").cloned()
            }

            fn get_key_page(next: std::collections::HashMap<String, String>) -> Option<String> {
                next.get("page").cloned()
            }
            loop {
                let page = client::headers::link(&h)
                    .and_then(get_key_next)
                    .and_then(get_key_page);
                if page.is_some() {
                    let (headers, _, res) = self.fetch_open_issues_with_pages("crvshlab", "ciot-backoffice", &page.unwrap()).unwrap();
                    issues_mut.append(res.unwrap().as_mut());
                    h = headers;
                } else {
                    break;
                }
            };
            println!(
                "Showing {} open issues in {}",
                num_total_issues, repo.full_name
            );
        }

        println!("");

        issues_mut.to_std_out();
    }
}
