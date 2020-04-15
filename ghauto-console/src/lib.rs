// #![recursion_limit="4096"]
#[macro_use]
extern crate clap;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate prettytable;
extern crate termion;
extern crate chrono;
extern crate ghauto_client_v3 as client;
extern crate ghauto_config as config;
extern crate itertools;
extern crate serde;
extern crate serde_json;
extern crate toml;
extern crate regex;

pub mod commands;
pub mod display;
pub mod cmd;
pub mod cli;

pub fn run() {
    cli::start();
}
