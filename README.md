# Swerve
_Quick and easy multithreaded file server for local development_

## About
A common misconception about swerve is that the name is a play on the word "serve", as in the purpose of a file server.
Contrary to this, swerve actually stands for "Simple Werve" - the goal being to make it super simple to werve your 
files from anywhere on the filesystem.

While swerve does nothing innovative, the goal is to have a fast performing file server with minimal set up and
tear down time, as well as minimal request time spent inside the server itself. Current features are limited to
serving files, but the roadmap includes API mocking to make front end development even easier.

## Installation

### Prebuilt
1. Download the binary for your OS from the [releases](https://github.com/Commander-lol/rust-swerve/releases) page
2. Put `swerve` somewhere in your `$PATH`, or modify your `$PATH` to inlcude the folder containing `swerve`
3. Do one of two things:
	1. Open up a command prompt and `cd` to the folder containing your files and run `swerve`
		```bash
		cd "$HOME/projects/my_awesome_site"
		swerve
		```
	2. Open up a command prompt and run `swerve`, specifying your target directory
		```bash
		swerve -d "$HOME/projects/my_awesome_site"
		```
		
### From source
To build and install swerve from source, you will need `rustc 1.27.0-nightly` or later as well as the Cargo package 
manager

1. In a command prompt, run `cargo install swerve`

This will build a copy of swerve on your local machine in case there isn't a binary built for your platform.
A little bit anticlimactic, but pretty simple.

### Usage
You can print out the usage text at any time by running `swerve -h` or `swerve --help`. The currently supported options
are:

Option | Param | Description | Notes
-------|-------|-------------|--------
`-h`, `--help` | | Print out the help text | 
`-d`, `--dir` | path; `string` | Use the target directory as the root for serving files | File paths are jailed, and will not be able to escape the root directory. Defaults to the directory in which the `swerve` command was run
`-p`, `--port` | port; `unsigned int` | Run the server listening on the given port | Defaults to `8200`
`-a`, `--address` | address; `string` | Bind the server to this network address, allowing remote connections | Defaults to `localhost`, preventing remote connections
`-t`, `--threads` | thread count; `unsigned int` | Create this number of threads in the thread pool, to use for serving files concurrently | Defaults to `32`. Performance may be improved on lower end machines by decreasing the number of worker threads
`-q`, `--quiet` | | Don't print anything to stdout | Useful if you're spawning swerve from another process and need to monitor stdout
`--no-index` | | Don't attempt to serve an `index.html` file from a directory path | By default, swerve will treat directories as requests for an `index.html` file. e.g. `/foo/bar` is treated as `/foo/bar/index.html`
`-u`, `--upload` | | Accept form uploads via a `POST` request to the `/uploads` endpoint | The `file_handling` and `field_handling` config value affect how forms are handled. Setting them to `Log` or `File` will log the values or write them to disk respectively
`-U`, `--upload-path` | path; `string` | Accept formuploads via a `POST` request to the specified endpoint | See the `--upload` option for information on configuration
`--license` | | Print out the GPL 3 license text | GPL compliance is cool, kids
