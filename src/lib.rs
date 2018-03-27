#![feature(match_default_bindings)]
#![feature(const_atomic_usize_new)]
#![feature(plugin)]
#![plugin(clippy)]

#[macro_use] extern crate error_chain;
#[macro_use]extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate toml;
extern crate dlc_decrypter;
extern crate reqwest;
extern crate regex;
extern crate md5;

pub mod error;
pub mod config;
pub mod package;
pub mod shareonline;
pub mod manager;
pub mod writer;
pub mod downloader;

pub use error::Error;
