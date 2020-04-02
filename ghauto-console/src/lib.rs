// #![recursion_limit="4096"]

#[macro_use]
extern crate serde_derive;
extern crate clap;
extern crate termion;
#[macro_use]
extern crate prettytable;
extern crate ghauto_client_v3 as client;
extern crate ghauto_config as config;
extern crate itertools;
extern crate serde;
extern crate serde_json;
extern crate toml;
extern crate reqwest;
extern crate chrono;

use clap::App;

use client::client::Github;
use config::context::BardoContext;

pub mod commands;

use commands::issues::get::GetIssuesCommand;
use commands::labels::get::GetLabelsCommand;
use commands::users::Command;

pub fn run() {
    let matches = App::new("bardo")
        .version("0.0.1")
        .author("Sebastian Kaiser")
        .about("The caretaker provides automations and more")
        .subcommand(
            App::new("gh")
                .about("Github repo automations")
                .subcommand(
                    App::new("issue")
                        .about("helpers for issues issues")
                        .subcommand(App::new("ls").about("iterates over all open issues")),
                )
                .subcommand(App::new("project").about("projects and more"))
                .subcommand(
                    App::new("repo")
                        .about("repo and more")
                        .subcommand(App::new("init").about("initializes a repo with defaults")),
                )
                .subcommand(App::new("check").about("Performs checks on configered repos")),
        )
        .subcommand(
            App::new("test")
                .about("some test")
                .subcommand(App::new("emails").about("emails"))
                .subcommand(App::new("labels").about("labels")),
        )
        .arg("-p, --profile 'overwrite profile value'")
        .get_matches();

    let context = BardoContext::init().unwrap();
    let access_token = &context
        .credentials()
        .profiles()
        .get("default")
        .unwrap()
        .access_token()
        .unwrap()
        .0;
    let gh = Github::new(access_token);

    match matches.subcommand() {
        ("gh", Some(gh_matches)) => match gh_matches.subcommand() {
            ("issue", Some(issue_matches)) => match issue_matches.subcommand() {
                ("ls", Some(ls_matches)) => match ls_matches.subcommand() {
                    ("", None) => GetIssuesCommand::new(context, gh).run(),
                    ("", Some(_)) => {
                        println!("ls subcommands");
                    }
                    _ => unreachable!(),
                },
                _ => {
                    println!("list all open issues");
                }
            },
            ("project", Some(project_matches)) => {
                println!("project cmds");
            }
            ("repo", Some(repo_matches)) => match repo_matches.subcommand() {
                ("init", Some(_)) => {
                    println!("repo cmds");
                    println!("push labels");
                }
                _ => unreachable!(),
            },
            ("check", Some(check_matches)) => {
                println!("check cmds");
            }
            _ => unreachable!(),
        },
        ("test", Some(test_matches)) => match test_matches.subcommand() {
            ("emails", Some(_)) => Command::new(context, gh).run(),
            ("labels", Some(_)) => GetLabelsCommand::new(context, gh).run(),
            _ => unreachable!(),
        },
        ("", None) => println!("No subcommand was used"),
        _ => unreachable!(),
    };
}
