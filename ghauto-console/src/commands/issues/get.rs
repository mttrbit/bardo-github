use client::client::{Executor, Github};
use config::context::BardoContext;

use itertools::Itertools;
use prettytable::{format, Cell, Row, Table};
use termion::{color, style};

#[derive(Deserialize, Debug)]
pub struct IssueLabel {
    name: String,
    color: String,
}

#[derive(Deserialize, Debug)]
pub struct IssueUser {
    login: String,
}

#[derive(Deserialize, Debug)]
pub struct Issue {
    number: i32,
    title: String,
    user: IssueUser,
    labels: Vec<IssueLabel>,
    assignees: Vec<IssueUser>,
    state: String,
    repository_url: String,
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
    fn print_assignees(v: &Vec<IssueUser>) -> String {
        if (v.len() > 3) {
            return format!(
                "{}, ..+{}",
                v.iter().take(3).map(|i| &i.login).join(", "),
                v.len() - 3
            );
        } else {
            return v.iter().map(|i| &i.login).join(", ");
        }
    }

    fn extract_from_url(url: &str) -> String {
        let mut s = String::from(url);
        let offset = 29;
        s.replace_range(..offset, "");
        s
    }

    fn print_labels(v: &Vec<IssueLabel>) -> String {
        if (v.len() > 3) {
            return format!(
                "{}, ... +{}",
                v.iter().take(3).map(|i| &i.name).join(", "),
                v.len() - 3
            );
        } else {
            return v.iter().map(|i| &i.name).join(", ");
        }
    }
}

impl<'a> PrintStd for IssuesPrinter<'a> {
    fn to_std_out(&self) {
        let mut table = Table::new();
        let format = format::FormatBuilder::new().padding(1, 1).build();
        table.set_format(format);

        table.set_titles(row![
            format!(
                "{}{}id{}",
                style::Bold,
                color::Fg(color::White),
                style::Reset
            ),
            format!(
                "{}{}org/name{}",
                style::Bold,
                color::Fg(color::Blue),
                style::Reset
            ),
            format!(
                "{}{}title{}",
                style::Bold,
                color::Fg(color::Magenta),
                style::Reset
            ),
            format!(
                "{}{}creator{}",
                style::Bold,
                color::Fg(color::Blue),
                style::Reset
            ),
            format!(
                "{}{}state{}",
                style::Bold,
                color::Fg(color::Red),
                style::Reset
            ),
            format!(
                "{}{}labels{}",
                style::Bold,
                color::Fg(color::White),
                style::Reset
            ),
            format!(
                "{}{}assignees{}",
                style::Bold,
                color::Fg(color::LightCyan),
                style::Reset
            ),
        ]);
        for e in (self.0).iter() {
            table.add_row(row![
                format!("{}{}{}", color::Fg(color::White), e.number, style::Reset),
                format!(
                    "{}{}{}",
                    color::Fg(color::LightBlue),
                    IssuesPrinter::extract_from_url(&e.repository_url),
                    style::Reset
                ),
                format!("{}{}{}", color::Fg(color::Magenta), e.title, style::Reset),
                format!(
                    "{}{}{}",
                    color::Fg(color::LightBlue),
                    e.user.login,
                    style::Reset
                ),
                format!("{}{}{}", color::Fg(color::LightRed), e.state, style::Reset),
                format!(
                    "{}({}){}",
                    color::Fg(color::White),
                    IssuesPrinter::print_labels(&e.labels),
                    style::Reset
                ),
                format!(
                    "{}{}{}",
                    color::Fg(color::LightCyan),
                    IssuesPrinter::print_assignees(&e.assignees),
                    style::Reset
                ),
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

    pub fn run(&self) {
        let (headers, status_code, res) = self
            .gh
            .get()
            .issues()
            // .execute::<serde_json::Value>()
            .execute::<Vec<Issue>>()
            .unwrap();

        let issues: Vec<Issue> = res.unwrap();
        issues.to_std_out();
    }
}
