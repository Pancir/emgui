use crate::experiments::test_3::{Derive, Widget};
use std::any::Any;
use std::cell::RefCell;
use std::rc::{Rc, Weak};

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct PushButton {
   self_ref: Weak<RefCell<Widget<Self>>>,
}

impl PushButton {
   pub fn new() -> Rc<RefCell<Widget<Self>>> {
      Widget::new(|_vt, self_ref| Self { self_ref })
   }
}

impl Derive for PushButton {
   fn as_any(&self) -> &dyn Any {
      self
   }

   fn as_any_mut(&mut self) -> &mut dyn Any {
      self
   }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
