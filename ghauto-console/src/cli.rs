use clap::ArgMatches;
use client::client::Github;
use config::context::BardoContext;

use crate::commands::issues::get::GetIssuesCommand;
use crate::commands::labels::get::GetLabelsCommand;
use crate::commands::users::Command;

pub fn start() {
    let matches = clap_app!(
        bardo =>
            (version: "0.0.1")
            (author: "Sebastian Kaiser")
            (about: "The caretaker provides automations and more")
            (@arg PROFILE: -p --profile "sets profile to use")
            (@subcommand gh =>
             (about: "repository automations for Github")
             (@subcommand issue =>
              (about: "helpers for dealing with issues")
              (@subcommand ls =>
               (about: "iterates over open issues")
               (@arg ALL: -a --all "fetches all issues from all registered projects")
               (@arg REPO: -r --repo +takes_value "fetches all issues from single project")
               (@arg FORMAT: -f --format +takes_value "define the print format")
              )
             )
             (@subcommand project =>
              (about: "helpers for dealing with projects")
             )
             (@subcommand repo =>
              (about: "helpers for dealing with repositories")
              (@subcommand ls =>
               (about: "list all repositories as defined in config")
              )
              (@subcommand new =>
               (about: "create a new repository and add it to your profile")
               (@arg REPO: -r --repo +takes_value "name of teh repository you want to create. format: organization/name")
              )
             )
             (@subcommand check =>
              (about: "performs checks")
             )
             (@subcommand status =>
              (about: "displays current status")
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

    let all_args = vec!["ALL", "REPO", "ORG", "NAME", "FORMAT"];

    fn get_args<'a>(matches: &'a ArgMatches, all_args: &Vec<&'a str>) -> Vec<Vec<&'a str>> {
        let mut args = Vec::new();
        for a in all_args.iter() {
            if matches.is_present(*a) {
                match matches.value_of(*a) {
                    Some(val) => {
                        let mut vals = Vec::new();
                        vals.push(*a);
                        vals.push(val);
                        args.push(vals);
                    }
                    None => {
                        let mut vals = Vec::new();
                        vals.push(*a);
                        args.push(vals);
                    }
                }
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
            ("project", Some(_project_matches)) => {
                println!("project cmds");
            }
            ("repo", Some(repo_matches)) => match repo_matches.subcommand() {
                ("init", Some(_)) => {
                    println!("repo cmds");
                    println!("push labels");
                }
                _ => unreachable!(),
            },
            ("check", Some(_check_matches)) => {
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
