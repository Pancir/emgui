use super::{
   events::{LifecycleEventCtx, LifecycleState},
   IWidget, StateFlags, WidgetId,
};
use sim_draw::m::Rect;
use std::{
   cell::{BorrowError, BorrowMutError, Ref, RefCell, RefMut},
   rc::{Rc, Weak},
};
use thiserror::Error;

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
            widget.on_lifecycle(&event);
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
   pub(crate) fn widget(&self) -> Result<Ref<'_, dyn IWidget>, BorrowError> {
      self.rc.try_borrow()
   }

   #[inline]
   pub(crate) fn widget_mut(&self) -> Result<RefMut<'_, dyn IWidget>, BorrowMutError> {
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
}

#[derive(Error, Debug)]
pub enum WidgetRefAccessErr {
   #[error("Widget is deleted")]
   Deleted,

   #[error(transparent)]
   Borrow(#[from] BorrowError),
}

impl WidgetRef {
   //    #[inline]
   //    pub(crate) fn widget(&self) -> Result<WidgetRefAccess<'_, dyn IWidget>, WidgetRefAccessErr> {
   //       let Some(w) = self.w.upgrade() else {
   //          return Err(WidgetRefAccessErr::Deleted);
   //       };

   //       match w.try_borrow() {
   //          Ok(b) => Ok(WidgetRefAccess { rc: w, r: b, _m: Default::default() }),
   //          Err(e) => return Err(WidgetRefAccessErr::from(e)),
   //       }
   //    }

   //    #[inline]
   //    pub(crate) fn widget_mut(&self) -> Result<WidgetRefAccess<'_, dyn IWidget>, WidgetRefAccessErr> {
   //       unimplemented!()
   //    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

// pub struct WidgetRefAccess<'widget, W>
// where
//    W: IWidget + ?Sized,
// {
//    rc: Rc<RefCell<W>>,
//    r: Ref<'widget, W>,
//    _m: PhantomData<&'widget W>,
// }

// impl<'widget, W> std::ops::Deref for WidgetRefAccess<'widget, W>
// where
//    W: IWidget,
// {
//    type Target = dyn IWidget;

//    fn deref(&self) -> &Self::Target {
//       self.r.deref()
//    }
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
