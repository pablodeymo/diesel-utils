#![warn(warnings, rust_2018_idioms, unsafe_code)]
#[macro_use]
extern crate log;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

pub mod connection;
pub mod paginate;
pub use connection::*;
pub use paginate::*;
