use super::FocusManager;
use crate::defines::{DEFAULT_DOUBLE_CLICK_TIME, DEFAULT_TOOL_TIP_TIME};
use crate::theme::Theme;
use std::cell::Cell;
use std::rc::Rc;
use std::time::Duration;

////////////////////////////////////////////////////////////////////////////////////////////////////

struct InnerRuntime {
   focus: FocusManager,
   theme: Theme,
   tool_type_time: Cell<Duration>,
   double_click_time: Cell<Duration>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

/// Contains shared data and setting for all widgets.
#[derive(Clone)]
pub struct Runtime {
   inner: Rc<InnerRuntime>,
}

impl Default for Runtime {
   fn default() -> Self {
      Self::new(Theme::default())
   }
}

impl Runtime {
   pub fn new(theme: Theme) -> Self {
      Self {
         inner: Rc::new(InnerRuntime {
            theme,
            focus: FocusManager::new(),
            tool_type_time: Cell::new(DEFAULT_TOOL_TIP_TIME),
            double_click_time: Cell::new(DEFAULT_DOUBLE_CLICK_TIME),
         }),
      }
   }

   #[inline]
   pub fn theme(&self) -> &Theme {
      &self.inner.theme
   }

   #[inline]
   pub fn set_tool_type_time(&self, duration: Duration) {
      self.inner.tool_type_time.set(duration);
   }

   #[inline]
   pub fn tool_type_time(&self) -> Duration {
      self.inner.tool_type_time.get()
   }

   #[inline]
   pub fn set_double_click_time(&self, duration: Duration) {
      self.inner.double_click_time.set(duration);
   }

   #[inline]
   pub fn double_click_time(&self) -> Duration {
      self.inner.double_click_time.get()
   }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
