use crate::experiments::test_3::{Derive, Widget};
use std::any::Any;
use std::cell::RefCell;
use std::rc::{Rc, Weak};

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct PushButton {
   self_ref: Option<Weak<RefCell<Widget<Self>>>>,
}

impl PushButton {
   pub fn new() -> Rc<RefCell<Widget<Self>>> {
      let s = Self { self_ref: None };

      let mut out = Widget::new(s, |vt| {});

      let self_ref = Rc::downgrade(&out);
      out.borrow_mut().derive.self_ref = Some(self_ref);

      out
   }
}

impl Derive for PushButton {
   fn as_any(&self) -> &dyn Any {
      self
   }

   fn set_ref(&mut self, self_ref: Weak<RefCell<Widget<Self>>>) {
      self.self_ref = Some(self_ref);
   }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
