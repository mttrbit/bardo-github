extern crate clap;

extern crate serde_json;
extern crate ghauto_config as config;
extern crate ghauto_client_v3 as client;

use clap::App;

use config::context::BardoContext;
use client::client::Github;

pub mod commands;

use commands::users::Command;

pub fn run() {
    let matches = App::new("bardo")
        .version("0.0.1")
        .author("Sebastian Kaiser <sebastian.kaiser@crvsh.io>")
        .about("Does awesome things")
        .arg("-c, --config=[FILE] 'Sets a custom config file'")
        .arg("<output> 'Sets an optional output file'")
        .arg("-d... 'Turn debugging information on'")
        .subcommand(
            App::new("gh")
                .about("provides github automations")
                .subcommand(App::new("test").about("authenticates with Github")),
        )
        .subcommand(
            App::new("test")
                .about("does testing things")
                .arg("-l, --list 'lists test values'"),
        )
        .get_matches();

    // You can check the value provided by positional arguments, or option arguments
    if let Some(o) = matches.value_of("output") {
        println!("Value for output: {}", o);
    }

    if let Some(c) = matches.value_of("config") {
        println!("Value for config: {}", c);
    }

    // You can see how many times a particular flag or argument occurred
    // Note, only flags can have multiple occurrences
    match matches.occurrences_of("d") {
        0 => println!("Debug mode is off"),
        1 => println!("Debug mode is kind of on"),
        2 => println!("Debug mode is on"),
        3 | _ => println!("Don't be crazy"),
    }

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level app
    if let Some(ref matches) = matches.subcommand_matches("gh") {
        // "$ myapp test" was run
        if matches.is_present("test") {

            let context = BardoContext::init().unwrap();
            let access_token = &context.credentials().profiles().get("default").unwrap().access_token().unwrap().0;
            let gh = Github::new(access_token);

            Command::new(context, gh).run();
            // "$ myapp test -l" was run
            println!("Authenticate with Github");
        }
    }

    if let Some(ref matches) = matches.subcommand_matches("check") {
        println!("Github authorize");
        //github_authorize();
    }
}
