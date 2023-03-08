use crate::core::widget_base::focus::FocusManager;
use crate::defines::{DEFAULT_DOUBLE_CLICK_TIME, DEFAULT_TOOL_TIP_TIME};
use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::time::Duration;

////////////////////////////////////////////////////////////////////////////////////////////////////

struct InnerRuntime {
   focus: FocusManager,
   tool_type_time: Cell<Duration>,
   double_click_time: Cell<Duration>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

/// Contains shared data and setting for all widgets.
#[derive(Clone)]
pub struct Runtime {
   inner: Rc<RefCell<InnerRuntime>>,
}

impl Runtime {
   pub fn new() -> Self {
      Self {
         inner: Rc::new(RefCell::new(InnerRuntime {
            focus: FocusManager::new(),
            tool_type_time: Cell::new(DEFAULT_TOOL_TIP_TIME),
            double_click_time: Cell::new(DEFAULT_DOUBLE_CLICK_TIME),
         })),
      }
   }

   pub fn set_tool_type_time(&self, duration: Duration) {
      self.inner.borrow_mut().tool_type_time.set(duration);
   }

   pub fn tool_type_time(&self) -> Duration {
      self.inner.borrow_mut().tool_type_time.get()
   }

   pub fn set_double_click_time(&self, duration: Duration) {
      self.inner.borrow_mut().double_click_time.set(duration);
   }

   pub fn double_click_time(&self) -> Duration {
      self.inner.borrow_mut().double_click_time.get()
   }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
