use rlua::{Lua, Result as LuaResult, FromLua};
use rocket::{Outcome, http};
use rocket::request::{FromRequest, Request};
use std::convert::{Into, AsRef, AsMut};
use scripting;
use serde::Serialize;

const LIB_JSON_ENCODE: &'static str = include_str!("../scripts/json.lua");

pub struct LuaRuntime(Lua);
impl Into<Lua> for LuaRuntime {
    fn into(self) -> Lua {
        self.0
    }
}
impl AsRef<Lua> for LuaRuntime {
    fn as_ref(&self) -> &Lua {
        &self.0
    }
}
impl AsMut<Lua> for LuaRuntime {
    fn as_mut(&mut self) -> &mut Lua {
        &mut self.0
    }
}

impl <'a, 'req>FromRequest<'a, 'req> for LuaRuntime {
    type Error = ();

    fn from_request(_request: &'a Request<'req>) -> Outcome<Self, (http::Status, ()), ()> {
        match create_runtime(false) {
			Ok(runtime) => Outcome::Success(runtime),
			_ => Outcome::Failure((http::Status::raw(500), ())),
		}
    }
}

pub fn create_runtime(with_debug: bool) -> LuaResult<LuaRuntime> {
    let runtime = if with_debug {
        unsafe { Lua::new_with_debug() }
    } else {
        Lua::new()
    };

	{
		runtime.eval::<()>(LIB_JSON_ENCODE, Some("json.lua".into()))?
	}

	{
		let globals = &runtime.globals();
		let response_constructor = runtime.create_function(|_, (status, content_type, body): (u16, String, String)| {
			Ok(scripting::ScriptResponse {
				status,
				content_type,
				body: Some(body),
			})
		})?;

		let empty_response_constructor = runtime.create_function(|_, (): ()| {
			Ok(scripting::ScriptResponse {
				status: 204,
				content_type: "text/plain".into(),
				body: None,
			})
		})?;

		globals.set("response", response_constructor)?;
		globals.set("empty_response", empty_response_constructor)?;
	}

    Ok(LuaRuntime(runtime))
}