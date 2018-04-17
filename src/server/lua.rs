use rlua::{Lua};
use rocket::{self, Outcome, http, Response};
use rocket::request::{FromRequest, Request};
use std::convert::{Into, AsRef, AsMut};

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
        Outcome::Success(create_runtime(false))
    }
}

pub fn create_runtime(with_debug: bool) -> LuaRuntime {
    let runtime = if with_debug {
        unsafe { Lua::new_with_debug() }
    } else {
        Lua::new()
    };

    // Customise runtime here

    LuaRuntime(runtime)
}