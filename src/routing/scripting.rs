use routing::request;
use server::LuaRuntime;
use rlua::{Lua};

#[get("/__testing__/run-script")]
pub fn route_script(path: request::RequestPath, runtime: LuaRuntime) -> String {
	let _lua: Lua = runtime.into(); //todo: Use This
	let doowap = path.0;
	let foo = request::path::MatchablePath(String::from(doowap));
	let matches = foo.matches(String::from("/inspection/123"));
	println!("{:?}", matches);

	String::from("Yes")
}