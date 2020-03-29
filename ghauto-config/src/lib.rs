#![allow(dead_code)] // Until every starting struct gets used
#![deny(//missing_docs,
        unsafe_code,
        unused_import_braces,
        unused_qualifications)]

extern crate dirs_sys;
extern crate toml;
extern crate serde_derive;

pub mod file;
pub mod profile;
pub mod credentials;
pub mod config;
pub mod context;
