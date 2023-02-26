pub mod dispatch;

use crate::core::{Geometry, IWidget, WidgetId};
use crate::defines::{STATIC_CHILD_NUM, STATIC_REGIONS_NUM};
use bitflags::bitflags;
use sim_draw::m::Rect;
use smallvec::SmallVec;
use std::cell::{Cell, RefCell};
use std::rc::{Rc, Weak};

////////////////////////////////////////////////////////////////////////////////////////////////////

bitflags! {
   /// Control flow for events.
   struct ControlFlow: u16 {
      //-----------------------------

      /// Call update for current widget.
      const SELF_UPDATE     = 1<<0;

      /// Some children need update.
      const CHILDREN_UPDATE = 1<<1;

      /// All update flags. Can be used to clear for example.
      const UPDATE = Self::SELF_UPDATE.bits | Self::CHILDREN_UPDATE.bits;

      //-----------------------------

      /// Call update for current widget.
      const SELF_DRAW     = 1<<2;

      /// Some children need update.
      const CHILDREN_DRAW = 1<<3;

      /// All update flags. Can be used to clear for example.
      const DRAW = Self::SELF_DRAW.bits | Self::CHILDREN_DRAW.bits;

      //-----------------------------

      /// Request to delete this widget.
      const SELF_DELETE     = 1<<4;

      /// Widget has one or more children to delete.
      const CHILDREN_DELETE = 1<<5;

      /// All delete flags. Can be used to clear for example.
      const DELETE = Self::SELF_DELETE.bits | Self::CHILDREN_DELETE.bits;

      //-----------------------------

      /// Visible state.
      const IS_VISIBLE = 1<<6;

      /// Visible state.
      const IS_ENABLED = 1<<7;

      /// Visible state.
      const IS_TRANSPARENT = 1<<8;

      //-----------------------------

      const INIT = Self::DRAW.bits | Self::UPDATE.bits | Self::IS_VISIBLE.bits | Self::IS_ENABLED.bits;
      const UPDATE_OR_DLETE = Self::DELETE.bits | Self::UPDATE.bits;
   }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub(crate) type ChildrenVec = SmallVec<[Rc<RefCell<dyn IWidget>>; STATIC_CHILD_NUM]>;
pub(crate) type RegionsVec = SmallVec<[Rect<f32>; STATIC_REGIONS_NUM]>;

pub struct Internal {
   parent: Option<Weak<RefCell<dyn IWidget>>>,
   pub(crate) geometry: Geometry,
   //--------------------
   control_flow: Cell<ControlFlow>,
   //--------------------
   draw_regions_busy: Cell<WidgetId>,
   draw_regions: RefCell<RegionsVec>,
   //--------------------
   children_busy: Cell<WidgetId>,
   children: RefCell<ChildrenVec>,
}

impl Default for Internal {
   fn default() -> Self {
      Self::new()
   }
}

impl Internal {
   pub(crate) fn new() -> Self {
      Self {
         parent: Default::default(),
         geometry: Geometry::default(),
         //--------------------
         control_flow: Cell::new(ControlFlow::INIT),
         //--------------------
         draw_regions_busy: Cell::new(WidgetId::INVALID),
         draw_regions: Default::default(),
         //--------------------
         children_busy: Cell::new(WidgetId::INVALID),
         children: Default::default(),
      }
   }
}

impl Internal {
   pub(crate) fn request_draw(&self) {
      let mut f = self.control_flow.get();

      if !f.contains(ControlFlow::SELF_DRAW) {
         f.set(ControlFlow::SELF_DRAW, true);
         self.control_flow.set(f);

         if let Some(parent) = &self.parent {
            if let Some(p) = parent.upgrade() {
               let mut bor = p.borrow_mut();
               let internal = bor.internal_mut();
               internal.control_flow.get_mut().set(ControlFlow::CHILDREN_DRAW, true);
               internal.draw_regions.get_mut().push(internal.geometry.rect())
            }
         }
      }
   }

   pub(crate) fn request_update(&self) {
      let mut f = self.control_flow.get();

      if !f.contains(ControlFlow::SELF_UPDATE) {
         f.set(ControlFlow::SELF_UPDATE, true);
         self.control_flow.set(f);

         if let Some(parent) = &self.parent {
            if let Some(p) = parent.upgrade() {
               let mut bor = p.borrow_mut();
               let internal = bor.internal_mut();
               internal.control_flow.get_mut().set(ControlFlow::CHILDREN_UPDATE, true);
            }
         }
      }
   }

   pub(crate) fn request_delete(&self) {
      let mut f = self.control_flow.get();

      if !f.contains(ControlFlow::SELF_DELETE) {
         f.set(ControlFlow::SELF_DELETE, true);
         self.control_flow.set(f);

         if let Some(parent) = &self.parent {
            if let Some(p) = parent.upgrade() {
               let mut bor = p.borrow_mut();
               let internal = bor.internal_mut();
               internal.control_flow.get_mut().set(ControlFlow::CHILDREN_DELETE, true);
            }
         }
      }
   }
}

impl Internal {
   pub(crate) fn is_visible(&self) -> bool {
      self.control_flow.get().contains(ControlFlow::IS_VISIBLE)
   }

   pub(crate) fn set_visible(&self, state: bool) {
      let mut f = self.control_flow.get();
      f.set(ControlFlow::IS_VISIBLE, state);
      self.control_flow.set(f);
   }

   pub(crate) fn is_enabled(&self) -> bool {
      self.control_flow.get().contains(ControlFlow::IS_ENABLED)
   }

   pub(crate) fn set_enabled(&self, state: bool) {
      let mut f = self.control_flow.get();
      f.set(ControlFlow::IS_ENABLED, state);
      self.control_flow.set(f);
   }

   pub(crate) fn is_transparent(&self) -> bool {
      self.control_flow.get().contains(ControlFlow::IS_TRANSPARENT)
   }

   pub(crate) fn set_transparent(&self, state: bool) {
      let mut f = self.control_flow.get();
      f.set(ControlFlow::IS_TRANSPARENT, state);
      self.control_flow.set(f);
   }
}

impl Internal {
   #[track_caller]
   pub(crate) fn take_regions(&mut self, id: WidgetId) -> RegionsVec {
      debug_assert!(
         !self.draw_regions_busy.get().is_valid(),
         "[{:?}] regions taken by [{:?}]",
         id,
         self.draw_regions_busy.get()
      );
      if !self.draw_regions_busy.get().is_valid() {
         self.draw_regions_busy.set(id);
      }
      std::mem::take(self.draw_regions.get_mut())
   }

   #[track_caller]
   pub(crate) fn set_regions(&mut self, mut ch: RegionsVec, id: WidgetId, clear: bool) {
      debug_assert!(
         self.draw_regions_busy.get().is_valid(),
         "[{:?}] attempt to set regions into non free slot",
         id
      );
      *self.draw_regions.get_mut() = std::mem::take(&mut ch);
      self.draw_regions_busy.set(WidgetId::INVALID);
      if clear {
         self.draw_regions.get_mut().clear();
      }
   }
}

impl Internal {
   #[track_caller]
   pub(crate) fn take_children(&mut self, id: WidgetId) -> ChildrenVec {
      debug_assert!(
         !self.children_busy.get().is_valid(),
         "[{:?}] children taken by [{:?}]",
         id,
         self.children_busy.get()
      );
      if !self.children_busy.get().is_valid() {
         self.children_busy.set(id);
      }
      std::mem::take(self.children.get_mut())
   }

   #[track_caller]
   pub(crate) fn set_children(&mut self, mut ch: ChildrenVec, id: WidgetId) {
      debug_assert!(
         self.children_busy.get().is_valid(),
         "[{:?}] attempt to set children into non free slot",
         id
      );
      *self.children.get_mut() = std::mem::take(&mut ch);
      self.children_busy.set(WidgetId::INVALID);
   }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

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
      let pch = bor.internal();

      debug_assert!(!pch.children_busy.get().is_valid());

      let mut bor_ch = pch.children.borrow_mut();
      bor_ch.push(child.clone());
   }

   {
      let mut bor = child.borrow_mut();
      let c = bor.internal_mut();
      c.parent = Some(Rc::downgrade(&parent));
   }

   w
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
   use super::*;

   #[test]
   fn sizes() {
      println!("{} : {}", std::any::type_name::<Internal>(), std::mem::size_of::<Internal>());
   }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
