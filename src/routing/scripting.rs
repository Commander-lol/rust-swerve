use scripting::run_script;
use std::path::PathBuf;

#[get("/__testing__/run-script")]
pub fn route_script() -> String {
	let path = PathBuf::from("example/.swerve/get_user_by_id.rhai");
	run_script(path).unwrap_or(String::from("No script"))
}