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

pub struct WidgetRefOwner {
   // TODO maybe Pin?
   rc: Rc<RefCell<dyn IWidget>>,
}

impl WidgetRefOwner {
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

impl WidgetRefOwner {
   #[inline]
   pub fn as_ref(&self) -> WidgetRef {
      WidgetRef { w: Rc::downgrade(&self.rc) }
   }

   #[inline]
   pub fn add_children<const NUM: usize>(&self, children: [WidgetRefOwner; NUM]) {
      // TODO swap implementation with add_child, it will be faster as we can only once borrow.

      for c in children {
         self.add_child(c);
      }
   }

   #[inline]
   pub fn add_child(&self, child: WidgetRefOwner) -> WidgetRef {
      super::add_child(self, child)
   }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone)]
pub struct WidgetRef {
   w: Weak<RefCell<dyn IWidget>>,
}

impl WidgetRef {
   pub(crate) fn upgrade(&self) -> Option<WidgetRefOwner> {
      self.w.upgrade().map(|v| WidgetRefOwner { rc: v })
   }
}

impl WidgetRef {
   #[inline]
   pub fn access(&self) -> Option<WidgetRefAccess<'_, dyn IWidget>> {
      self.w.upgrade().map(|rc| WidgetRefAccess { rc, _m: Default::default() })
   }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct WidgetRefAccess<'widget, W>
where
   W: IWidget + ?Sized,
{
   rc: Rc<RefCell<W>>,
   _m: std::marker::PhantomData<&'widget W>,
}

impl<'widget, W> WidgetRefAccess<'widget, W>
where
   W: IWidget,
{
   #[inline]
   pub fn widget(&self) -> Result<Ref<'_, W>, BorrowError> {
      self.rc.try_borrow()
   }

   #[inline]
   pub fn widget_mut(&self) -> Result<RefMut<'_, W>, BorrowMutError> {
      self.rc.try_borrow_mut()
   }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
