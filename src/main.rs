#![feature(plugin)]
#![feature(alloc_system)]
#![plugin(rocket_codegen)]

extern crate alloc_system;
extern crate rocket;
extern crate docopt;
extern crate swerve;
extern crate rhai;

use std::process;
use docopt::Docopt;
use swerve::cli;
use swerve::server;

fn main() {
    let args: cli::Args = Docopt::new(cli::USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    let is_quiet = args.flag_quiet;
    macro_rules! printq {
        ($( $x:expr ),+) => {
            {
                if !is_quiet {
                    println!($($x),*);
                }
            }
        }
    }

    if args.flag_help {
        printq!("{}", cli::USAGE);
        process::exit(0);
    }

	if args.flag_license {
        if is_quiet { std::process::exit(0); }
		cli::gpl::show_license_and_exit();
	}

    let config_path = args.get_dir().join(".swerve/config.yml");
    let swerve_config = cli::SwerveConfig::from_file(&config_path).unwrap_or_else(|e| {
        println!("Error in config file {} | {}", config_path.to_string_lossy(), e);
        std::process::exit(2);
    });

	let server = server::create_server(args.clone(), swerve_config.clone());
	server.launch();
}