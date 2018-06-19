use server::LuaRuntime;
use rlua::{Lua};
use scripting::{run_script, ScriptResponse};

use rocket::request::{FromForm, FormItems};
use std::collections::HashMap;

#[derive(Debug)]
pub struct ScriptParams {
	pub script_name: String,
	pub script_params: HashMap<String, String>,
}

impl <'form> FromForm<'form> for ScriptParams {
	type Error = ();

	fn from_form(items: &mut FormItems<'form>, _: bool) -> Result<ScriptParams, Self::Error> {
		let mut script_name: Option<String> = None;
		let mut script_params: HashMap<String, String> = HashMap::new();

		for (key, value) in items {
			match key.as_str() {
				"script_path" if script_name.is_none() => { script_name = Some(String::from(value.as_str())); },
				_ => { script_params.insert(String::from(key.as_str()), String::from(value.as_str())); },
			};
		}

		match script_name {
			Some(name) => Ok(ScriptParams { script_name: name, script_params }),
			None => Err(()),
		}
	}
}


#[get("/__run_script__?<params>")]
pub fn route_script(params: ScriptParams, runtime: LuaRuntime) -> ScriptResponse {
	let lua: Lua = runtime.into();
	run_script(format!("example/.swerve/{}", params.script_name), &lua, params.script_params).unwrap_or_else(|| ScriptResponse::default())
}
