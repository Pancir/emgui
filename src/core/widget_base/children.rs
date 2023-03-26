use anyhow::bail;

use super::ChildrenVec;
use crate::core::{IWidget, WidgetId};
use std::{
   cell::{Cell, Ref, RefCell},
   rc::Rc,
};

////////////////////////////////////////////////////////////////////////////////////////////////////

/// Entity to work with widget children.
///
/// It solves the problem when children list should be changed while it is being iterated.
/// The solution is to create a list of actions to process when iteration is completed.
pub struct Children {
   #[cfg(debug_assertions)]
   borrowed_at: Cell<Option<&'static core::panic::Location<'static>>>,
   children_borrowed: Cell<WidgetId>,

   parent: Option<Weak<RefCell<dyn IWidget>>>,
   children: RefCell<ChildrenVec>,
}

impl Children {
   pub(crate) fn iter(&self) -> anyhow::Result<ChildrenIter> {
      match self.children.try_borrow() {
         Ok(borrow) => {
            #[cfg(debug_assertions)]
            {
               self.borrowed_at.set(Some(core::panic::Location::caller()));
            }

            Ok(ChildrenIter { ch: self, vec: borrow })
         }
         Err(_) => {
            #[cfg(debug_assertions)]
            bail!(
               "Children already borrowed by: {:?} at location: {}",
               self.children_borrowed.get(),
               self.borrowed_at.get().unwrap()
            );

            #[cfg(not(debug_assertions))]
            bail!("Children already borrowed by: {:?}", self.children_borrowed.get());
         }
      }
   }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub(crate) struct ChildrenIter<'a> {
   ch: &'a Children,
   vec: Ref<'a, ChildrenVec>,
}

impl<'a> std::ops::Drop for ChildrenIter<'a> {
   fn drop(&mut self) {}
}

impl<'a> core::iter::Iterator for ChildrenIter<'a> {
   type Item = &'a Rc<RefCell<dyn IWidget>>;

   fn next(&mut self) -> Option<Self::Item> {
      todo!()
   }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
