use clap::ArgMatches;
use client::client::Github;
use config::context::BardoContext;
use std::env;

use crate::commands::issues::get::GetIssuesCommandExecutor;
use crate::commands::labels::get::GetLabelsCommand;
use crate::commands::pulls::get::GetPullsCommand;
use crate::commands::repo::clone::CloneRepoCommandExecutor;
use crate::commands::repo::apply::ApplyCommand;
use crate::commands::users::Command;
use crate::cmd::CommandExecutor;

enum CliOpts {
    ALL,
    REPO,
    ORG,
    NAME,
    FORMAT,
    PROFILE,
}

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

fn resolve_profile_argument<'a>(matches: &'a ArgMatches) -> Option<String> {
    let arg = vec!["PROFILE"];
    let v = get_args(matches, &arg); // [[PROFILE name]]
    if !v.is_empty() && v[0].len() == 2 {
        return Some(v[0][1].to_string());
    } else {
        return None;
    }
}

fn resolve_profile<'a>(matches: &'a ArgMatches) -> String {
    env::var_os("BARDO_DEFAULT_PROFILE")
        .map(|s| std::ffi::OsString::into_string(s).unwrap())
        .or_else(|| resolve_profile_argument(matches))
        .or_else(|| Some("default".to_string()))
        .expect("failed to resolve the profile")
}

pub fn start() {
    let matches = clap_app!(
        bardo =>
            (version: "0.0.1")
            (author: "Sebastian Kaiser")
            (about: "The caretaker provides automations and more")
            (@arg PROFILE: -p --profile +takes_value +global "sets profile to use")
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
              (@subcommand status =>
               (about: "displays a summary of issues")
              )
             )
             (@subcommand pr =>
              (about: "iterates over all repositories to display open pull requests")
              (@subcommand ls =>
               (about: "iterates over open pull requets")
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
              (@subcommand clone =>
               (about: "iterates over all repositories to clone them in clone_path")
               (@arg REPO: -r --repo +takes_value "fetches all issues from single project")
              )
              (@subcommand apply =>
               (about: "iterates over all configured repositories, clones each of them into a temporary folder, and runs command")
               (@arg BRANCH: -b --branch +takes_value +required "the name of the branch to which the change is committed.")
               (@arg MESSAGE: -m --message +takes_value +required "the commit message to use")
               (@arg COMMENT: -c --comment +takes_value +required "the comment to use for the new pull request")
               (@arg REVIEWERS: --reviewers +takes_value "the reviewer(s) to assign to the pull request")
               (@arg CMD: +required "the shell script to apply")
               (@arg REPO: -r --repo +takes_value "use a single project")
              )
              (@subcommand create =>
               (about: "create a new repository and add it to your profile")
               (@arg REPO: +required "name of the repository you want to create. format: organization/name")
               (@arg DEFAULTS: -d --defaults "creates a new repository and sets defaults")
              )
             )
             (@subcommand check =>
              // Checks might be:
              // CODEOWNERS set
              // templates set
              // license set
              // project is linked to a team
              // etc.
              (about: "performs checks")
             )
             (@subcommand status =>
              (about: "displays current status")
             )
            )
    )
    .get_matches();

    let default_profile = resolve_profile(&matches);

    let context = BardoContext::init(&default_profile).unwrap();
    let access_token = &context
        .credentials()
        .profiles()
        .get(&default_profile)
        .unwrap()
        .access_token()
        .unwrap()
        .0;
    let gh = Github::new(access_token);

    let all_args = vec![
        "ALL",
        "REPO",
        "ORG",
        "NAME",
        "FORMAT",
        "PROFILE",
        "BRANCH",
        "MESSAGE",
        "COMMENT",
        "REVIEWERS",
        "CMD",
    ];

    match matches.subcommand() {
        ("gh", Some(gh_matches)) => match gh_matches.subcommand() {
            ("issue", Some(issue_matches)) => match issue_matches.subcommand() {
                ("ls", Some(ls_matches)) => {
                    let args = get_args(ls_matches, &all_args);
                    GetIssuesCommandExecutor::new(gh, context).execute(&args);
                }
                _ => unreachable!(),
            },
            ("pr", Some(pr_matches)) => match pr_matches.subcommand() {
                ("ls", Some(ls_matches)) => {
                    let args = get_args(ls_matches, &all_args);
                    GetPullsCommand::new(context, gh).run(&args);
                }
                _ => unreachable!(),
            },
            ("project", Some(_project_matches)) => {
                println!("project cmds");
            }
            ("repo", Some(repo_matches)) => match repo_matches.subcommand() {
                ("create", Some(_)) => {
                    println!("repo cmds");
                    println!("push labels");
                }
                ("clone", Some(clone_matches)) => {
                    let args = get_args(clone_matches, &all_args);
                    CloneRepoCommandExecutor::new(context).execute(&args);
                }
                ("apply", Some(apply_matches)) => {
                    let args = get_args(apply_matches, &all_args);
                    ApplyCommand::new(context, gh).run(&args);
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
