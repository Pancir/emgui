use crate::core::{IWidget, WidgetId};
use crate::defines::STATIC_CHILD_NUM;
use smallvec::SmallVec;
use std::{
   cell::{Cell, Ref, RefCell, UnsafeCell},
   rc::{Rc, Weak},
};

////////////////////////////////////////////////////////////////////////////////////////////////////

enum Commands {
   Parent,
   Add,
   Delete,
   Raise,
   Lower,
}

pub(crate) type ChildrenVec = SmallVec<[Rc<RefCell<dyn IWidget>>; STATIC_CHILD_NUM]>;

////////////////////////////////////////////////////////////////////////////////////////////////////

/// Entity to work with widget children.
///
/// It solves the problem when children list should be changed while it is being iterated.
/// The solution is to create a list of actions to process when iteration is completed.
pub struct Children {
   children_busy: Cell<WidgetId>,
   children: RefCell<ChildrenVec>,
}

impl Children {
   pub fn raise(&self, w: Weak<RefCell<dyn IWidget>>) {
      unimplemented!()
   }

   pub fn lower(&self, w: Weak<RefCell<dyn IWidget>>) {
      unimplemented!()
   }

   pub fn add(&self, w: Weak<RefCell<dyn IWidget>>) {
      unimplemented!()
   }

   pub fn delete(&self, w: Weak<RefCell<dyn IWidget>>) {
      unimplemented!()
   }

   pub fn is_under_process(&self) -> bool {
      unimplemented!()
   }
}

impl Default for Children {
   fn default() -> Self {
      Self { children_busy: Cell::new(WidgetId::INVALID), children: Default::default() }
   }
}

impl Children {
   #[track_caller]
   pub(crate) fn take_children(&mut self, id: WidgetId) -> ChildrenVec {
      debug_assert!(
         !self.children_busy.get().is_valid(),
         "[{:?}] children borrowed by [{:?}]",
         id,
         self.children_busy.get()
      );

      if !self.children_busy.get().is_valid() {
         self.children_busy.set(id);
      }
      std::mem::take(self.children.get_mut())
   }

   #[track_caller]
   pub(crate) fn return_children(&mut self, mut ch: ChildrenVec, id: WidgetId) {
      debug_assert!(
         self.children_busy.get().is_valid(),
         "[{:?}] attempt to release borrowed children that already released.",
         id
      );

      *self.children.get_mut() = std::mem::take(&mut ch);
      self.children_busy.set(WidgetId::INVALID);
   }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
