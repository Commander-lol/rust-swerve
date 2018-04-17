use rocket::{self, Rocket, Config};
use cli::{Args, SwerveConfig};
use routing;

pub fn create_server(args: Args, config: SwerveConfig) -> Rocket {
	let server_config = server_config_from_input(args.clone(), config.clone());
	let mut server = Rocket::custom(server_config, false)
		.manage(args.clone())
		.manage(config.clone());

	let quiet = args.flag_quiet;

    if let Some(ref upload_path) = args.flag_upload_path {
        if !quiet { println!("[SETUP] Accepting uploads at {}", upload_path) }
        server = server.mount(upload_path, routes![routing::mock_upload::to_file]);
    } else if args.flag_upload {
        if !quiet { println!("[SETUP] Accepting uploads at /upload") }
        server = server.mount("/upload", routes![routing::mock_upload::to_file]);
    }

	server = server.mount("/", routes![
		routing::core::serve_root,
		routing::core::serve_files,
		routing::scripting::route_script
	]);


	if !quiet {
		server = server.attach(rocket::fairing::AdHoc::on_launch(move |rckt| {
			let conf = rckt.config();
			println!("[SETUP] Swerve is configured with {} worker threads", conf.workers);
			println!("[SETUP] Swerving files from http://{}:{}\n", conf.address, conf.port);
		}))
		.attach(rocket::fairing::AdHoc::on_response(|req, _res| {
			println!("[REQUEST] {} {}", req.method(), req.uri());
		}));
	}

	server
}

fn server_config_from_input(args: Args, config: SwerveConfig) -> Config {
    let mut builder = Config::build(rocket::config::Environment::Development);
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