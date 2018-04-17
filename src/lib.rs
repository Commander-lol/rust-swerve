#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate serde_yaml;
extern crate rocket;
extern crate rocket_contrib;
extern crate formdata;
extern crate hyper;
extern crate rand;

pub mod cli;
pub mod routing;
pub mod scripting;
pub mod server;
