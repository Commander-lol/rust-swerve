use rlua::{UserData, UserDataMethods, Table};
use rocket::Request;
use rocket::http::Status;
use rocket::response::{Response, Responder, ResponseBuilder};
use std::default::Default;
use std::io::Cursor;

#[derive(Debug, Clone)]
pub struct ScriptResponse {
	pub status: u16,
	pub content_type: String,
	pub body: Option<String>,
}

impl Default for ScriptResponse {
	fn default() -> Self {
		ScriptResponse {
			status: 500,
			content_type: "text/plain".into(),
			body: Some("Failed to return resposne from script".into()),
		}
	}
}



impl UserData for ScriptResponse {
	fn add_methods(methods: &mut UserDataMethods<Self>) {
		methods.add_method_mut("set_status", |_, response: &mut ScriptResponse, status: u16| {
			response.status = status;
			Ok(())
		})
	}
}

impl <'r>Responder<'r> for ScriptResponse {
	fn respond_to(self, _: &Request) -> Result<Response<'r>, Status> {
		let mut r = Response::build();
		r.status(Status::raw(self.status));
		r.raw_header("Content-Type", self.content_type);
		if let Some(body) = self.body {
			r.sized_body(Cursor::new(body));
		}
		r.ok()
	}
}