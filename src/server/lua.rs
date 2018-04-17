use rlua::{Lua};

pub fn create_runtime(with_debug: bool) -> Lua {
    let runtime = if with_debug {
        unsafe { Lua::new_with_debug() }
    } else {
        Lua::new()
    };

    // Customise runtime here

    runtime
}