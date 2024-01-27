#![allow(clippy::missing_errors_doc)]

use std::time::{Duration, Instant};

use async_io::{block_on, Timer};

use mlua::prelude::*;
use mlua_luau_runtime::Runtime;

const MAIN_SCRIPT: &str = include_str!("./lua/basic_sleep.luau");

pub fn main() -> LuaResult<()> {
    // Set up persistent Lua environment
    let lua = Lua::new();
    lua.globals().set(
        "sleep",
        lua.create_async_function(|_, duration: f64| async move {
            let before = Instant::now();
            let after = Timer::after(Duration::from_secs_f64(duration)).await;
            Ok((after - before).as_secs_f64())
        })?,
    )?;

    // Load the main script into a runtime
    let rt = Runtime::new(&lua);
    let main = lua.load(MAIN_SCRIPT);
    rt.spawn_thread(main, ())?;

    // Run until completion
    block_on(rt.run());

    Ok(())
}

#[test]
fn test_basic_sleep() -> LuaResult<()> {
    main()
}
