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
extern crate futures;
extern crate http;
extern crate hyper;
extern crate oauth2;
extern crate serde;
extern crate serde_json;
extern crate tokio;

pub mod client;
pub mod errors;
pub mod gh_auth;

pub use hyper::{HeaderMap, StatusCode};
