# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

## [0.3.0]

### Added
- [DEV] Initial test scaffolding
- Lua scripting support as a route handler in the `config.yml` file

## [0.2.0]

### Added
- Support for `-u, --uploads` to add a path that accepts form submissions
- Support for -U=<path>, --upload-path=<path>` to add a custom path that accepts form submissions
- `.swerve` configuration directory support. Swerve will look for a `.swerve` directory in the target directory, containing a `config.yml` file that can define various server options, including command line options
- Support "Log" and "File" handlers for files in form submissions
- Support "Log" handler for fields in form submissions
- [UNSTABLE] Initial support for custom route scripting

### Fixed
- Incorrect file type returned for `.wasm` files

## [0.1.1]

### Fixed
- [DEV] Minor errors in metadata files

## [0.1.0]

### Added
- Serve files from the current directory
- Serve files from the target directory
- Serve index files from directories
- Configure number of worker threads
- Bind to custom address and port
