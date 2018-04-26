use std::convert::AsRef;
use std::path::Path;
use std::fs::File;
use std::io::Read;

pub fn run_script<P: AsRef<Path>>(path: P) -> Option<String> {
    let mut file = File::open(&path).unwrap();
    let mut buf = String::new();

    match file.read_to_string(&mut buf) {
		Ok(_) => {},
		Err(_) => return None,
	}

    Some(buf)
}