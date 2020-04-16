use crate::cmd::{Command, IterableCommand, HttpResponse, ResultIterator, PrintStd};
use crate::commands::repo::get::{GetRepoCmd, Repository};
use client::client::{Executor, Github, Result};
use config::context::BardoContext;

use prettytable::{format, Table};
use termion::{color, style};

#[derive(Deserialize, Debug)]
pub struct Head {
    label: String,
}

#[derive(Deserialize, Debug)]
pub struct Pull {
    number: i32,
    title: String,
    updated_at: String,
    head: Head,
}

pub struct GetPullsCommand {
    context: BardoContext,
    gh: Github,
}

impl GetPullsCommand {
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
            self.get_pulls(org, name, print_all);
        } else {
            let profile = self.context.profile();
            let repositories = self.context.config().get_profiles()[profile].repositories();
            for r in repositories.iter() {
                match (r.org(), r.name()) {
                    (o, Some(n)) => self.get_pulls(&o.0, &n.0, print_all),
                    (_, _) => (),
                };
            }
        }
    }

    fn get_pulls(&self, org: &str, name: &str, b_print_all: bool) {
        let cmd: FetchOpenPullsCmd = FetchOpenPullsCmd::new(&self.gh, org, name);
        let (_, _, repo_res) = GetRepoCmd(&self.gh, org, name).execute().unwrap();
        let repo: Repository = repo_res.unwrap();
        let full_name = repo.full_name();
        let mut pulls_mut: Vec<Pull>;

        let mut iter = cmd.execute_iter().into_iter();

        if b_print_all == false {
            let (_, _, res) = iter.next().unwrap().unwrap();
            pulls_mut = res.unwrap();
            let num_fetched_pulls = pulls_mut.len();
            if num_fetched_pulls > 0 {
                println!(
                    "Showing {} open pull requests in {}",
                    num_fetched_pulls, full_name
                );
                println!("");
            } else {
                println!("There are no open pull requests in {}", full_name);
            }
        } else {
            pulls_mut = Vec::new();
            for next in iter {
                let (_, _, res) = next.unwrap();
                pulls_mut.append(res.unwrap().as_mut());
            }

            println!(
                "Showing {} open pull requests in {}",
                pulls_mut.len(), full_name
            );

            println!("");
        }

        pulls_mut.to_std_out();
    }
}

struct Pulls<'a>(&'a Vec<Pull>);

impl<'a> PrintStd for Vec<Pull> {
    fn to_std_out(&self) {
        Pulls(self).to_std_out();
    }
}

impl<'a> PrintStd for Pulls<'a> {
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
                "{}{}label{}",
                style::Bold,
                color::Fg(color::White),
                style::Reset
            ),
        ]);

        for e in v.iter() {
            table.add_row(row![
                format!("{}#{}{}", color::Fg(color::Green), e.number, style::Reset),
                format!("{}{}{}", color::Fg(color::Magenta), e.title, style::Reset),
                format!("{}{}{}", color::Fg(color::White), e.head.label, style::Reset),
            ]);
        }
        table.printstd();
    }
}

pub struct FetchOpenPullsCmd<'a> {
    gh: &'a Github,
    owner: &'a str,
    name: &'a str,
}

impl<'a> FetchOpenPullsCmd<'a> {
    fn new(gh: &'a Github, owner: &'a str, name: &'a str) -> Self {
        Self {
            gh: gh,
            owner: owner,
            name: name,
        }
    }
}

impl<'a> IterableCommand<Vec<Pull>> for FetchOpenPullsCmd<'a> {
    fn execute_iter(&self) -> ResultIterator<Vec<Pull>> {
        fn call<'a>(
            gh: &'a Github,
            owner: &'a str,
            name: &'a str,
        ) -> Box<dyn Fn(&str) -> Result<HttpResponse<Vec<Pull>>> + 'a> {
            Box::new(move |page| {
                gh.get()
                    .repos()
                    .owner(owner)
                    .repo(name)
                    .pulls()
                    .page(page)
                    .execute::<Vec<Pull>>()
            })
        }

        let t = call(self.gh, self.owner, self.name);
        ResultIterator::new(t, Some("1".to_string()))
    }
}
