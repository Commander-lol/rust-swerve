pub const USAGE: &'static str = "
Static file swerver and api mocker for local development. For full documentation,
visit https://swerve.louiscap.io.

Usage:
    swerve [options]

Options:
    -h, --help                      Display this text
    -q, --quiet                     Don't print anything to stdout
    --no-index                      Don't serve index.html files for directory paths
    -d=<path>, --dir=<path>         Root directory; defaults to cwd
    -p=<port>, --port=<port>        Port; defaults to 8200
    -a=<addr>, --address=<addr>     Address to bind to (e.g. 10.0.0.15); defaults to localhost
    -c=<path>, --config=<path>      Path to the .swerve config file
    -t=<num>, --threads=<num>       Number of worker threads to use for serving files; defaults to 32
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
}