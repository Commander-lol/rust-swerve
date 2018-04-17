#![feature(plugin)]
#![plugin(interpolate_idents)]

extern crate rocket;
extern crate swerve;

use swerve::cli::{Args, SwerveConfig};
use swerve::server::create_server;

use rocket::local::Client;
use rocket::http::ContentType;

macro_rules! test_type {
    ($name:ident, $path:expr, $content_path:expr) => (interpolate_idents! {
        #[test]
        fn [returns_some_type_for_ $name]() {
            let args = Args {
                flag_dir: Some(String::from("example")),
                flag_quiet: true,
                ..Args::default()
            };
            let config = SwerveConfig::default();

            let server = create_server(args, config);
            let client = Client::new(server).expect("valid server instance");
            let response = client.get($path).dispatch();

            assert_eq!(response.content_type(), Some($content_path));
        }
    });

    ($name:ident, $path:expr) => (interpolate_idents! {
        #[test]
        fn [returns_no_type_for_ $name]() {
            let args = Args {
                flag_dir: Some(String::from("example")),
                flag_quiet: true,
                ..Args::default()
            };
            let config = SwerveConfig::default();

            let server = create_server(args, config);
            let client = Client::new(server).expect("valid server instance");
            let response = client.get($path).dispatch();

            assert_eq!(response.content_type(), None);
        }
    });
}

test_type!(html, "/index.html", ContentType::HTML);
test_type!(css, "/css/styles.css", ContentType::CSS);
test_type!(javascript, "/js/say_hello.js", ContentType::JavaScript);
test_type!(csv, "/files/attribution.csv", ContentType::CSV);
test_type!(image_jpeg, "/files/adorable-puppy.jpg", ContentType::JPEG);
test_type!(web_assembly, "/files/math.wasm", ContentType::new("application", "wasm"));
