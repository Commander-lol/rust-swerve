use rlua::{UserData, UserDataMethods};

use rocket::Request;
use rocket::http::Status;
use rocket::response::{Response, Responder};

use std::io::Cursor;
use std::default::Default;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ScriptResponse {
	pub status: u16,
	pub headers: HashMap<String, String>,
	pub body: Option<String>,
}

impl Default for ScriptResponse {
	fn default() -> Self {
		ScriptResponse {
			status: 500,
			headers: HashMap::new(),
			body: Some("Failed to return response from script".into()),
		}
	}
}

impl UserData for ScriptResponse {
	fn add_methods(methods: &mut UserDataMethods<Self>) {
		methods.add_method_mut("set_status", |_, response: &mut ScriptResponse, status: u16| {
			response.status = status;
			Ok(())
		});
		methods.add_method_mut("set_header", |_, response: &mut ScriptResponse, (header, value): (String, String)| {
			response.headers.insert(header, value);
			Ok(())
		});
		methods.add_method_mut("set_body", |_, response: &mut ScriptResponse, body: String| {
			response.body = Some(body);
			Ok(())
		});
		methods.add_method_mut("unset_body", |_, response: &mut ScriptResponse, (): ()| {
			response.body = None;
			Ok(())
		});
	}
}

impl <'r>Responder<'r> for ScriptResponse {
	fn respond_to(self, _: &Request) -> Result<Response<'r>, Status> {
		let mut r = Response::build();
		r.status(Status::raw(self.status));
		for (name, value) in self.headers {
			r.raw_header(name, value);
		}
		if let Some(body) = self.body {
			r.sized_body(Cursor::new(body));
		}
		r.ok()
	}
}
