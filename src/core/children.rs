use crate::core::{IWidget, WidgetId};
use crate::defines::STATIC_CHILD_NUM;
use smallvec::SmallVec;
use std::cell::{Cell, RefCell};
use std::ops::{Deref, DerefMut};
use std::rc::{Rc, Weak};

////////////////////////////////////////////////////////////////////////////////////////////////////

pub type ChildrenVec = SmallVec<[Rc<RefCell<dyn IWidget>>; STATIC_CHILD_NUM]>;

#[derive(Default)]
pub struct Children {
   parent: RefCell<Option<Weak<RefCell<dyn IWidget>>>>,

   children_busy: Cell<WidgetId>,
   children: RefCell<ChildrenVec>,
}

impl Children {
   pub(crate) fn request_draw_parent(&self) {
      let bor = self.parent.borrow_mut();
      if let Some(p) = bor.deref() {
         if let Some(o) = p.upgrade() {
            o.borrow().request_draw();
         }
      }
   }

   pub(crate) fn request_update_parent(&self) {
      let bor = self.parent.borrow_mut();
      if let Some(p) = bor.deref() {
         if let Some(o) = p.upgrade() {
            o.borrow().request_update();
         }
      }
   }

   #[track_caller]
   pub(crate) fn take(&self, id: WidgetId) -> ChildrenVec {
      debug_assert!(
         !self.children_busy.get().is_valid(),
         "[{:?}] children taken by [{:?}]",
         id,
         self.children_busy.get()
      );
      if !self.children_busy.get().is_valid() {
         self.children_busy.set(id);
      }
      std::mem::take(self.children.borrow_mut().deref_mut())
   }

   #[track_caller]
   pub(crate) fn set(&self, mut ch: ChildrenVec, id: WidgetId) {
      debug_assert!(
         self.children_busy.get().is_valid(),
         "[{:?}] attempt to set children into non free slot",
         id
      );
      *self.children.borrow_mut().deref_mut() = std::mem::take(&mut ch);
      self.children_busy.set(WidgetId::INVALID);
   }
}

pub fn add_children<const NUM: usize>(
   parent: &Rc<RefCell<dyn IWidget>>,
   children: [Rc<RefCell<dyn IWidget>>; NUM],
) {
   // TODO swap implementation with add_child, it will be faster as we can only once borrow.

   for c in children {
      add_child(&parent, c);
   }
}

pub fn add_child(
   parent: &Rc<RefCell<dyn IWidget>>,
   child: Rc<RefCell<dyn IWidget>>,
) -> Weak<RefCell<dyn IWidget>> {
   let w = Rc::downgrade(&child);
   {
      let bor = parent.borrow();
      let pch = bor.children();

      debug_assert!(!pch.children_busy.get().is_valid());

      let mut bor_ch = pch.children.borrow_mut();
      bor_ch.push(child.clone());
   }

   {
      let bor = child.borrow();
      let c = bor.children();

      let mut bor_p = c.parent.borrow_mut();
      debug_assert!(bor_p.is_none());
      *bor_p.deref_mut() = Some(Rc::downgrade(&parent));
   }

   w
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
   use super::*;

   #[test]
   fn sizes() {
      println!("{} : {}", std::any::type_name::<Children>(), std::mem::size_of::<Children>());
   }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
