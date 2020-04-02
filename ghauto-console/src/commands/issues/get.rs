use client::client::{Executor, Github, Result};
use config::context::BardoContext;

use itertools::Itertools;
use prettytable::{format, Cell, Row, Table};
use termion::{color, style};
use reqwest::header::HeaderMap;
use reqwest::StatusCode;
use chrono::{DateTime, TimeZone, NaiveDateTime, Utc, Local, Duration, FixedOffset};
use std::str::FromStr;
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
        format!("{} {}{}", num, unit, if num == 1 { "s" } else { "" })
    }
   
    fn fuzzy_ago(ago: Duration) -> String {
        if ago.num_seconds() < 60 {
		    return "less than a minute ago".to_string()
	    }
	    if ago.num_minutes() < 60 {
		    return IssuesPrinter::format_duration(ago.num_minutes(), "minute");
	    }
	    if ago.num_hours() < 24 {
		    return IssuesPrinter::format_duration(ago.num_hours(), "hour");
	    }
	    if ago.num_hours() < 720 {
		    return IssuesPrinter::format_duration(ago.num_hours()/24, "day");
	    }
	    if ago.num_hours() < 262800 {
		    return IssuesPrinter::format_duration(ago.num_hours()/720, "month");
	    }

	    IssuesPrinter::format_duration(ago.num_hours()/262800, "year")
    }

    fn print_last_updated(now: &DateTime<Utc>, updated_at: &DateTime<FixedOffset>) -> String {
        let ago: Duration =  now.signed_duration_since(*updated_at);
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
            let updated_at = IssuesPrinter::print_last_updated(&now, &dt_8601);
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

    fn fetch_repo_data(&self, owner: &str, repo: &str) -> Result<(HeaderMap, StatusCode, Option<Repo>)> {
       self.gh
            .get()
            .repos()
            .owner(owner)
            .repo(repo)
            // .execute::<serde_json::Value>()
            .execute::<Repo>()
    }

    fn fetch_open_issues(&self, owner: &str, repo: &str) -> Result<(HeaderMap, StatusCode, Option<Vec<Issue>>)> {
        self.gh
            .get()
            .repos()
            .owner(owner)
            .repo(repo)
            .issues()
            // .execute::<serde_json::Value>()
            .execute::<Vec<Issue>>()
    }

    pub fn run(&self) {
        let (headers, status_code, res) = self.fetch_open_issues("crvshlab", "ciot-backoffice").unwrap();
        let (_, _, repo_res) = self.fetch_repo_data("crvshlab", "ciot-backoffice").unwrap();

        println!("headers: {:#?}", repo_res);
        println!("{:#?}", client::headers::link(&headers));

        let repo = repo_res.unwrap();
        let issues: Vec<Issue> = res.unwrap();

        println!("");
        println!("Showing {} of {} open issues in {}", issues.len(), repo.open_issues_count, repo.full_name);
        println!("");

        issues.to_std_out();
    }
}
