#![feature(plugin)]
#![feature(alloc_system)]
#![plugin(rocket_codegen)]

extern crate alloc_system;
extern crate rocket;
extern crate docopt;
extern crate swerve;
extern crate rhai;

use rhai::Engine;

use std::{path, process, io};
use std::fs::{self, File};
use docopt::Docopt;
use rocket::response::NamedFile;
use rocket::http::ContentType;
use rocket::{Response, Request};
use swerve::cli;
use swerve::routing;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use rocket::response::Responder;

struct TypedFile {
    file: File,
    content_type: Option<ContentType>,
    path: PathBuf,
}

impl TypedFile {
    pub fn open<P: AsRef<Path>>(path: P, content_type: Option<rocket::http::ContentType>) -> TypedFile {
        let file = File::open(path.as_ref()).unwrap();
        TypedFile { file, content_type, path: (*path.as_ref()).to_path_buf() }
    }
}

impl rocket::response::Responder<'static> for TypedFile {
    fn respond_to(self, _: &Request) -> Result<Response<'static>, rocket::http::Status> {
        let mut response = Response::new();
        if let Some(content_type) = self.content_type {
            response.set_header(content_type);
        } else {
            response.set_header(ContentType::from_extension(&self.path.extension().unwrap().to_string_lossy()).unwrap());
        }
        response.set_streamed_body(BufReader::new(self.file));
        Ok(response)
    }
}

#[get("/")]
fn serve_root(args: rocket::State<cli::Args>) -> Option<TypedFile> {
    serve_files(None, args)
}

#[get("/<file..>")]
fn serve_files(file: Option<path::PathBuf>, args: rocket::State<cli::Args>) -> Option<TypedFile> {
    let stub = match file {
        Some(path) => path,
        None => path::PathBuf::from(""),
    };

    let path = args.get_dir().join(stub);

    let meta = match fs::metadata(&path) {
        Ok(metadata) => metadata,
        _ => return None,
    };

    if meta.is_dir() && !args.flag_no_index {
        Some(TypedFile::open(path.join("index.html"), None))
    } else {
        if &path.extension().unwrap().to_string_lossy() == "wasm" {
           Some( TypedFile::open(path, Some(ContentType::new("application", "wasm"))))
        } else {
            Some(TypedFile::open(path, None))
        }
    }
}

fn config_from_args(args: cli::Args, config: cli::SwerveConfig) -> rocket::Config {
    let mut builder = rocket::Config::build(rocket::config::Environment::Development);
    if let Some(threads) = args.flag_threads {
        builder = builder.workers(threads);
    } else {
        builder = builder.workers(config.server.threads);
    }

    if let Some(port) = args.flag_port {
        builder = builder.port(port);
    } else {
        builder = builder.port(config.server.port);
    }

    if let Some(address) = args.flag_address {
        builder = builder.address(address);
    } else {
        builder = builder.address(config.server.address);
    }

    builder.finalize().unwrap()
}

fn main() {
    let args: cli::Args = Docopt::new(cli::USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    if args.flag_help {
        println!("{}", cli::USAGE);
        process::exit(0);
    }

	if args.flag_license {
		cli::gpl::show_license_and_exit();
	}

    let config_path = args.get_dir().join(".swerve/config.yml");
    let swerve_config = cli::SwerveConfig::from_file(&config_path).unwrap_or_else(|e| {
        println!("Error in config file {} | {}", config_path.to_string_lossy(), e);
        std::process::exit(2);
    });

    let server_config = config_from_args(args.clone(), swerve_config.clone());
    println!("{:?}", swerve_config);

    let mut server = rocket::custom(server_config, false)
        .manage(args.clone())
        .manage(swerve_config);

	
    server = server.mount("/upload", routes![swerve::routing::mock_upload::to_file])
        .mount("/", routes![serve_root, serve_files]);

    if !args.flag_quiet {
        server = server.attach(rocket::fairing::AdHoc::on_launch(move |rckt| {
            let config = rckt.config();
            println!("\nSwerve is configured with {} worker threads", config.workers);
            println!("Swerving files from http://{}:{}\n", config.address, config.port);
        }))
        .attach(rocket::fairing::AdHoc::on_response(|req, _res| {
            println!("{} {}", req.method(), req.uri());
        }));
    }
    {
        server.launch();
    }
}