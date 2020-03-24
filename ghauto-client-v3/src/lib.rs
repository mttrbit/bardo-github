//! Library to used to access the Github API with Rust
#![allow(dead_code)] // Until every starting struct gets used
#![deny(//missing_docs,
        unsafe_code,
        unused_import_braces,
        unused_qualifications)]

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate error_chain;

extern crate bytes;
extern crate hyper;
extern crate oauth2;
extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate tokio;
extern crate webbrowser;
extern crate dirs_sys;
extern crate toml;

extern crate ghauto_config;

#[macro_use]
mod macros;
mod util;

pub mod client;
pub mod errors;
pub mod gh_auth;

pub mod users;

pub use hyper::{HeaderMap, StatusCode};
