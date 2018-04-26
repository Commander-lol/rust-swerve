use std::convert::AsRef;
use std::path::Path;
use std::fs::File;
use std::io::Read;
use rlua::Lua;

pub fn run_script<P: AsRef<Path>>(path: P, mut lua: &Lua) -> Option<String> {
    let mut file = File::open(&path).unwrap();
    let mut buf = String::new();

    match file.read_to_string(&mut buf) {
		Ok(_) => {},
		Err(_) => return None,
	}

	println!("{}", buf);

	lua.eval::<()>(&buf, None);

    Some(buf)
}