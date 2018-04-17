
extern crate rocket;
extern crate swerve;

use swerve::cli::{Args, SwerveConfig};
use swerve::server::create_server;

use rocket::local::Client;
use rocket::http::Status;

const INDEX_PAGE: &'static str = include_str!("../example/index.html");

#[test]
fn test_serves_index() {
	let args = Args {
		flag_dir: Some(String::from("example")),
		..Args::default()
	};
	let config = SwerveConfig::default();

	let server = create_server(args, config);

	let client = Client::new(server).expect("valid server instance");
	let mut response = client.get("/").dispatch();

	assert_eq!(response.status(), Status::Ok);
	assert_eq!(response.body_string(), Some(INDEX_PAGE.into()));
}