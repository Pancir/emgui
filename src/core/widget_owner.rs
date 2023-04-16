use super::{
   events::{LifecycleEventCtx, LifecycleState},
   IWidget,
};
use std::{
   cell::RefCell,
   rc::{Rc, Weak},
};

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct WidgetOwner {
   rc: Rc<RefCell<dyn IWidget>>,
}

impl WidgetOwner {
   #[inline]
   pub fn new<W>(w: W) -> Self
   where
      W: IWidget + Sized,
   {
      let s = Rc::new(RefCell::new(w));
      let w = Rc::downgrade(&s);
      let w_ref = WidgetRef { w };

      match s.try_borrow_mut() {
         Ok(mut widget) => {
            let event = LifecycleEventCtx { state: LifecycleState::SelfReference(w_ref) };
            widget.emit_lifecycle(&event);
         }
         Err(_) => {
            // # Safety
            // The widget is just created and owned by this function.
            unsafe { std::hint::unreachable_unchecked() };
         }
      }
      Self { rc: s }
   }

   #[inline]
   pub fn get_ref(&self) -> WidgetRef {
      WidgetRef { w: Rc::downgrade(&self.rc) }
   }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct WidgetRef {
   w: Weak<RefCell<dyn IWidget>>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
