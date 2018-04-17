use std::convert::AsRef;
use std::path::Path;
use std::fs::File;
use std::sync::Arc;
use std::io::Read;
use std::collections::HashMap;

pub fn run_script<P: AsRef<Path>>(path: P) -> Option<String> {
    let mut file = File::open(&path).unwrap();
    let mut buf = String::new();

    file.read_to_string(&mut buf);
    Some(buf)
}