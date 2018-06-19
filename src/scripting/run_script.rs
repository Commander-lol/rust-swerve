use std::convert::AsRef;
use std::path::Path;
use std::fs::File;
use std::io::Read;
use rlua::Lua;
use scripting::ScriptResponse;
use std::collections::HashMap;

pub fn run_script<P: AsRef<Path>>(path: P, lua: &Lua, params: HashMap<String, String>) -> Option<ScriptResponse> {
    let mut file = File::open(&path).unwrap();
    let mut buf = String::new();

    match file.read_to_string(&mut buf) {
		Ok(_) => {},
		Err(_) => return None,
	}

	let file_name = path.as_ref()
		.file_name()
		.and_then(|name| name.to_str());

	let params_table = serialize_table("params", params);

	lua.eval::<ScriptResponse>(&format!("{}\n{}", params_table, buf), file_name)
		.map_err(|e| println!("{}", e))
		.ok()
}

fn serialize_table(name: &'static str, table: HashMap<String, String>) -> String {
	let mut output = format!("local {} = {{", name);

	let contents = table.iter()
		.fold(
			String::new(),
			|acc, (key, value)| format!("{} {} = \"{}\",", acc, key, value)
		);

	output.push_str(&contents[0..contents.len() - 1]);
	output.push_str(" }");

	output
}
