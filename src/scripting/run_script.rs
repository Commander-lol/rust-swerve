use std::convert::AsRef;
use std::path::Path;
use std::fs::File;
use std::io::Read;
use rlua::{Lua, UserData};
use scripting::ScriptResponse;
use std::collections::HashMap;

pub fn run_script<P: AsRef<Path>>(path: P, mut lua: &Lua, params: HashMap<String, String>) -> Option<ScriptResponse> {
    let mut file = File::open(&path).unwrap();
    let mut buf = String::new();

    match file.read_to_string(&mut buf) {
		Ok(_) => {},
		Err(_) => return None,
	}

	let file_name = path.as_ref()
		.file_name()
		.and_then(|name| name.to_str());

	let params_table = lua.create_table().ok()?;
	for (key, value) in params {
		params_table.set(key, value);
	}

	// TODO: Inject params_table in a per-request manner

	params_table.
	lua.eval::<ScriptResponse>(&buf, file_name).map_err(|e| println!("{}", e)).ok()
}