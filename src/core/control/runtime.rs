use crate::core::control::focus::FocusManager;
use std::rc::Rc;

////////////////////////////////////////////////////////////////////////////////////////////////////

struct InnerRuntime {
   focus: FocusManager,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone)]
pub struct Runtime {
   inner: Rc<InnerRuntime>,
}

impl Runtime {
   pub fn new() -> Self {
      Self { inner: Rc::new(InnerRuntime { focus: FocusManager::new() }) }
   }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
