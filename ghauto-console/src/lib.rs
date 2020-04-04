// #![recursion_limit="4096"]
#[macro_use]
extern crate clap;
#[macro_use]
extern crate serde_derive;
extern crate termion;
#[macro_use]
extern crate prettytable;
extern crate chrono;
extern crate ghauto_client_v3 as client;
extern crate ghauto_config as config;
extern crate itertools;
extern crate serde;
extern crate serde_json;
extern crate toml;

use clap::ArgMatches;
use client::client::Github;
use config::context::BardoContext;

pub mod commands;
pub mod display;

use commands::issues::get::GetIssuesCommand;
use commands::labels::get::GetLabelsCommand;
use commands::users::Command;

pub fn run() {
    let matches = clap_app!(
        bardo =>
            (version: "0.0.1")
            (author: "Sebastian Kaiser")
            (about: "The caretaker provides automations and more")
            (@arg PROFILE: -p --profile +takes_value "sets profile to use")
            (@subcommand gh =>
             (about: "repository automations for Github")
             (@subcommand issue =>
              (about: "helpers for dealing with issues")
              (@subcommand ls =>
               (about: "iterates over open issues")
               (@arg ALL: -a --all "fetches all issues")
              )
             )
             (@subcommand project =>
              (about: "helpers for dealing with projects")
             )
             (@subcommand repo =>
              (about: "helpers for dealing with repositories")
              (@subcommand ls =>
               (about "list all repositories as defined in config")
              )
             )
             (@subcommand check =>
              (about: "performs checks")
             )
            )
    )
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

    let all_args = vec!["ALL"];

    fn get_args<'a>(matches: &ArgMatches, all_args: &Vec<&'a str>) -> Vec<&'a str> {
        let mut args = Vec::new();
        for a in all_args.iter() {
            if matches.is_present(*a) {
                args.push(*a);
            }
        }
        args
    }

    match matches.subcommand() {
        ("gh", Some(gh_matches)) => match gh_matches.subcommand() {
            ("issue", Some(issue_matches)) => match issue_matches.subcommand() {
                ("ls", Some(ls_matches)) => {
                    let args = get_args(ls_matches, &all_args);
                    GetIssuesCommand::new(context, gh).run(&args);
                }
                _ => unreachable!(),
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
