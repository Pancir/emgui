use super::FocusManager;
use crate::backend::Resources;
use crate::defines::{DEFAULT_DOUBLE_CLICK_TIME, DEFAULT_TOOL_TIP_TIME};
use crate::theme::Theme;
use std::cell::Cell;
use std::rc::Rc;
use std::time::Duration;

////////////////////////////////////////////////////////////////////////////////////////////////////

struct SharedDataInner {
   focus: FocusManager,
   theme: Theme,
   tool_type_time: Cell<Duration>,
   double_click_time: Cell<Duration>,
   resources: Box<dyn Resources>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

/// Contains shared data and setting for all widgets.
#[derive(Clone)]
pub struct SharedData {
   inner: Rc<SharedDataInner>,
}

impl SharedData {
   pub fn new<RES>(theme: Theme, resources: RES) -> Self
   where
      RES: Resources + 'static,
   {
      Self {
         inner: Rc::new(SharedDataInner {
            theme,
            focus: FocusManager::new(),
            tool_type_time: Cell::new(DEFAULT_TOOL_TIP_TIME),
            double_click_time: Cell::new(DEFAULT_DOUBLE_CLICK_TIME),
            resources: Box::new(resources),
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
