use super::{
   events::{LifecycleEventCtx, LifecycleState},
   IWidget, StateFlags, WidgetId,
};
use sim_draw::m::Rect;
use std::{
   cell::{BorrowError, BorrowMutError, Ref, RefCell, RefMut},
   rc::{Rc, Weak},
};

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct WidgetOwner {
   // TODO maybe Pin?
   rc: Rc<RefCell<dyn IWidget>>,
}

impl WidgetOwner {
   #[inline]
   pub(crate) fn new<W>(w: W) -> Self
   where
      W: IWidget,
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
   pub(crate) fn clone(&self) -> Self {
      Self { rc: self.rc.clone() }
   }

   #[inline]
   pub(crate) fn borrow(&self) -> Ref<'_, dyn IWidget> {
      self.try_borrow().unwrap()
   }

   #[inline]
   pub(crate) fn try_borrow(&self) -> Result<Ref<'_, dyn IWidget>, BorrowError> {
      self.rc.try_borrow()
   }

   #[inline]
   pub(crate) fn borrow_mut(&self) -> RefMut<'_, dyn IWidget> {
      self.try_borrow_mut().unwrap()
   }

   #[inline]
   pub(crate) fn try_borrow_mut(&self) -> Result<RefMut<'_, dyn IWidget>, BorrowMutError> {
      self.rc.try_borrow_mut()
   }

   #[inline]
   pub(crate) fn data_for_dispatcher(&self) -> (StateFlags, WidgetId, Rect<f32>, i8, bool) {
      // # Safety
      // It seems it is quite safe, we just read simple copyable variables.
      // Just in case in debug mode we check availability.
      debug_assert!(self.rc.try_borrow_mut().is_ok());

      unsafe { (*self.rc.as_ptr()).base().data_for_dispatcher() }
   }
}

impl WidgetOwner {
   #[inline]
   pub fn as_ref(&self) -> WidgetRef {
      WidgetRef { w: Rc::downgrade(&self.rc) }
   }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone)]
pub struct WidgetRef {
   w: Weak<RefCell<dyn IWidget>>,
}

impl WidgetRef {
   pub(crate) fn upgrade(&self) -> Option<WidgetOwner> {
      self.w.upgrade().map(|v| WidgetOwner { rc: v })
   }

   //    pub fn access(&self) -> Option<WidgetRefAccess<'_, dyn IWidget>> {
   //       match self.w.upgrade() {
   //          Some(w) => Some(WidgetRefAccess { rc: w, _m: Default::default() }),
   //          None => None,
   //       }
   //    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

// pub struct WidgetRefAccess<'widget, W>
// where
//    W: IWidget,
// {
//    rc: Rc<RefCell<W>>,
//    _m: PhantomData<&'widget W>,
// }

// impl<'widget, W> WidgetRefAccess<'widget, W>
// where
//    W: IWidget,
// {
//    pub fn widget(&self) -> Option<&dyn IWidget> {
//       self.w.upgrade()
//    }
// }

////////////////////////////////////////////////////////////////////////////////////////////////////
