use std::hash::{Hash, Hasher};

use mlua::prelude::*;

/**
    Opaque and unique ID representing a [`LuaThread`].

    Typically used for associating metadata with a thread in a structure such as a `HashMap<ThreadId, ...>`.

    Note that holding a `ThreadId` does not prevent the thread from being garbage collected.
    The actual thread may or may not still exist and be active at any given point in time.
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ThreadId(usize);

impl From<LuaThread<'_>> for ThreadId {
    fn from(thread: LuaThread) -> Self {
        Self(LuaValue::Thread(thread).to_pointer() as usize)
    }
}

impl From<&LuaThread<'_>> for ThreadId {
    fn from(thread: &LuaThread) -> Self {
        Self(LuaValue::Thread(thread.clone()).to_pointer() as usize)
    }
}

impl Hash for ThreadId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}