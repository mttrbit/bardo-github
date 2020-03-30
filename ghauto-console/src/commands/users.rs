use serde::de::DeserializeOwned;

use client::client::{Executor, Github};
use config::context::BardoContext;

use termion::{color, style};
use prettytable::{Table, Row, Cell, format};

#[derive(Deserialize, Debug)]
pub struct Email {
    email: String,
    primary: bool,
}

pub struct Command {
    context: BardoContext,
    gh: Github,
}

impl Command {
    pub fn new(ctx: BardoContext, gh: Github) -> Self {
        Self {
            context: ctx,
            gh: gh,
        }
    }

    pub fn run(&self) {
        let (headers, statusCode, res) = self
            .gh
            .get()
            .user()
            .emails()
            .execute::<Vec<Email>>()
            //.execute::<serde_json::Value>()
            //.execute::<Vec<User>>()
            .unwrap();
        let emails: Vec<Email> = res.unwrap();
        let mut table = Table::new();
        let format = format::FormatBuilder::new()
            .padding(1, 1)
            .build();
        table.set_format(format);

        table.set_titles(row![
            format!("{}Email{}", color::Fg(color::Blue), style::Reset),
            format!("{}Primary{}", color::Fg(color::Red), style::Reset)
        ]);
        for e in emails.iter() {
            table.add_row(row![
                format!("{}{}{}", color::Fg(color::Blue), e.email, style::Reset),
                format!("{}{}{}", color::Fg(color::Red), e.primary, style::Reset)
            ]);
        }
        table.printstd();
    }
}
