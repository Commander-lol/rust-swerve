#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate docopt;
extern crate swerve;

use std::{path, fs, process};
use docopt::Docopt;
use rocket::response::NamedFile;

use swerve::cli;

#[get("/")]
fn serve_root(args: rocket::State<cli::Args>) -> Option<NamedFile> {
    serve_files(None, args)
}

#[get("/<file..>")]
fn serve_files(file: Option<path::PathBuf>, args: rocket::State<cli::Args>) -> Option<NamedFile> {
    let stub = match file {
        Some(path) => path,
        None => path::PathBuf::from(""),
    };

    let path = match args.flag_dir {
        Some(ref root) => path::PathBuf::from(root).join(stub),
        None => stub,
    };

    let meta = match fs::metadata(&path) {
        Ok(metadata) => metadata,
        _ => return None,
    };

    if meta.is_dir() && !args.flag_no_index {
        NamedFile::open(path.join("index.html")).ok()
    } else {
        NamedFile::open(path).ok()
    }
}

fn config_from_args(args: cli::Args) -> rocket::Config {
    let mut builder = rocket::Config::build(rocket::config::Environment::Development);
    if let Some(threads) = args.flag_threads {
        builder = builder.workers(threads);
    } else {
        builder = builder.workers(32);
    }

    if let Some(port) = args.flag_port {
        builder = builder.port(port);
    } else {
        builder = builder.port(8200);
    }

    if let Some(address) = args.flag_address {
        builder = builder.address(address);
    } else {
        builder = builder.address("localhost");
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

    let config = config_from_args(args.clone());

    let mut server = rocket::custom(config, false)
        .manage(args.clone())
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

    server.launch();
}