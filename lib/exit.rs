use std::{cell::Cell, rc::Rc};

use event_listener::Event;

#[derive(Debug, Clone)]
pub(crate) struct Exit {
    code: Rc<Cell<Option<i32>>>,
    event: Rc<Event>,
}

impl Exit {
    pub fn new() -> Self {
        Self {
            code: Rc::new(Cell::new(None)),
            event: Rc::new(Event::new()),
        }
    }

    pub fn set(&self, code: i32) {
        self.code.set(Some(code));
        self.event.notify(usize::MAX);
    }

    pub fn get(&self) -> Option<i32> {
        self.code.get()
    }

    pub async fn listen(&self) {
        self.event.listen().await;
    }
}
