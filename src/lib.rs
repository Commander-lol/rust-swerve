#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use] extern crate serde_derive;
extern crate rocket;
extern crate rocket_contrib;
extern crate formdata;
extern crate hyper;

pub mod cli;
pub mod routing;