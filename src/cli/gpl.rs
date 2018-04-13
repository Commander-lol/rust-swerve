use std::process;

const LICENSE: &'static str = include_str!("../../COPYING");

pub fn show_license_and_exit() {
	println!("{}", LICENSE);
	process::exit(0);
}