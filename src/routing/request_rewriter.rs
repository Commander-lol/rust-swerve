use rocket::fairing::{Fairing, Info, Kind};
use rocket::{Request, Data, State};
use cli::SwerveConfig;
use std::collections::HashMap;
use routing::request::path::MatchablePath;

pub struct ScriptMap(pub HashMap<MatchablePath, String>);
impl ScriptMap {
	pub fn from_config(conf: &SwerveConfig) -> Self {
		let mut map: HashMap<MatchablePath, String> = HashMap::new();

		for handler in &conf.routes {
			if let Some(ref script) = handler.script {
				map.insert(MatchablePath::from(handler.route.clone()), script.clone());
			}
		}

		ScriptMap(map)
	}
}

pub struct RedirectScripts;
impl Fairing for RedirectScripts {
	fn info(&self) -> Info {
		Info {
			name: "Redirect Scripts To Handler",
			kind: Kind::Request,
		}
	}

	fn on_request(&self, request: &mut Request, _: &Data) {
		let script_map_container = request.guard::<State<ScriptMap>>().unwrap();
		let script_map: &HashMap<MatchablePath, String> = &script_map_container.0;
		for path in script_map.keys() {
			let matches = path.matches(request.uri().path());
			if let Some(values) = matches {
				let script_name = script_map.get(path).unwrap();

				let mut value_buffer = String::new();
				value_buffer.push_str("script_path=");
				value_buffer.push_str(script_name);

				for (ref param, ref val) in values {
					value_buffer.push_str("&");
					value_buffer.push_str(param);
					value_buffer.push_str("=");
					value_buffer.push_str(val);
				}

				let path = format!("/__run_script__/?{}", value_buffer);
				request.set_uri(path);
				break;
			}
		}
	}
}