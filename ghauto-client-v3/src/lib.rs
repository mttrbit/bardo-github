//! Library to used to access the Github API with Rust
#![allow(dead_code)] // Until every starting struct gets used
#![deny(//missing_docs,
        unsafe_code,
        unused_import_braces,
        unused_qualifications)]

#[macro_use]
extern crate error_chain;

extern crate serde;
extern crate serde_json;
extern crate futures;
extern crate oauth2;
extern crate tokio_core;
extern crate hyper;

#[macro_use]
mod macros;
mod util;

pub mod gh_auth;
pub mod client;
pub mod errors;

pub use hyper::{HeaderMap, StatusCode};
