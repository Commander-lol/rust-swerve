pub use std::path::PathBuf;
pub use std::env::current_dir;

pub const USAGE: &'static str = "
Static file swerver and api mocker for local development. 
For full documentation, visit https://swerve.louiscap.io.

Usage:
    swerve [options]

General Options:
    -h, --help                       Display this text
    -q, --quiet                      Don't print anything to stdout
    --license                        Show the GPL v3 license text

Web Server Options:
    --no-index                       Don't serve index.html files for directory paths
    -d=<path>, --dir=<path>          Root directory; defaults to cwd
    -p=<port>, --port=<port>         Port; defaults to 8200
    -a=<addr>, --address=<addr>      Address to bind to (e.g. 10.0.0.15); defaults to localhost
    -c=<path>, --config=<path>       Path to the .swerve config file
    -t=<num>, --threads=<num>        Number of worker threads to use for serving files; defaults to 32

Data Handling Options
    -u, --upload                     Support file uploads to '/upload'
    -U=<path>, --upload-path=<path>  Set the url path that will accept file uploads. Implies 'upload' flag if not present

Swerve Copyright (C) 2018  Louis Capitanchik
Licensed under GPLv3+
";

#[derive(Debug, Deserialize, Clone)]
pub struct Args {
    pub flag_dir: Option<String>,
    pub flag_port: Option<u16>,
    pub flag_config: Option<String>,
    pub flag_threads: Option<u16>,
    pub flag_address: Option<String>,
    pub flag_help: bool,
    pub flag_quiet: bool,
    pub flag_no_index: bool,
	pub flag_upload: bool,
	pub flag_upload_path: Option<String>,
	pub flag_license: bool,
}

impl Args {
    pub fn get_dir(&self) -> PathBuf {
        let dir_flag = self.flag_dir.clone();
        PathBuf::from(
            dir_flag.unwrap_or(
            current_dir().unwrap_or(
            PathBuf::from("")
            ).to_string_lossy().into_owned())
        )
    }
}