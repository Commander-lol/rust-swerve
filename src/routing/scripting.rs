use scripting::run_script;
use std::path::PathBuf;
use routing::request;
use server::LuaRuntime;
use rlua::{Lua};

#[get("/__testing__/run-script")]
pub fn route_script(path: request::RequestPath, runtime: LuaRuntime) -> String {
	let lua: Lua = runtime.into();
	let doowap = path.0;
	let foo = request::path::MatchablePath(String::from("/inspection/@id"));
	let matches = foo.matches(String::from("/inspection/123"));
	println!("{:?}", matches);
	let path = PathBuf::from("example/.swerve/get_user_by_id.rhai");
	run_script(path).unwrap_or(String::from("No script"))
}