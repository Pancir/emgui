mod dispatcher;
mod focus;
mod runtime;

pub use dispatcher::*;

////////////////////////////////////////////////////////////////////////////////////////////////////

use crate::core::control::runtime::Runtime;
use crate::core::{Geometry, IWidget, WidgetId};
use crate::defines::STATIC_CHILD_NUM;
use bitflags::bitflags;
use sim_draw::color::Rgba;
use smallvec::SmallVec;
use std::cell::{Cell, RefCell};
use std::rc::{Rc, Weak};

////////////////////////////////////////////////////////////////////////////////////////////////////

bitflags! {
   /// Control flow for events.
   struct StateFlags: u16 {
      //-----------------------------

      /// Call update for current widget.
      const SELF_UPDATE     = 1<<0;

      /// Some children need update.
      const CHILDREN_UPDATE = 1<<1;

      //-----------------------------

      /// Call update for current widget.
      const SELF_DRAW     = 1<<2;

      /// Some children need update.
      const CHILDREN_DRAW = 1<<3;

      //-----------------------------

      /// Request to delete this widget.
      const SELF_DELETE     = 1<<4;

      /// Widget has one or more children to delete.
      const CHILDREN_DELETE = 1<<5;

      //-----------------------------

      /// Visible state.
      const IS_VISIBLE = 1<<6;

      /// Set if interaction events are desired like mouse and keyboard ones..
      const IS_ENABLED = 1<<7;

      /// Set if background has transparent pixels.
      const IS_TRANSPARENT = 1<<8;

      //-----------------------------

      /// It is set when mouse is over the widgets rectangle.
      const IS_OVER = 1<<9;

      /// Mouse tracking state.
      const HAS_MOUSE_TRACKING = 1<<10;

      //-----------------------------

      const INIT = Self::SELF_DRAW.bits|Self::CHILDREN_DRAW.bits|Self::SELF_UPDATE.bits|Self::CHILDREN_UPDATE.bits|Self::IS_VISIBLE.bits|Self::IS_ENABLED.bits;
   }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub(crate) type ChildrenVec = SmallVec<[Rc<RefCell<dyn IWidget>>; STATIC_CHILD_NUM]>;

pub struct Internal {
   parent: Option<Weak<RefCell<dyn IWidget>>>,
   //--------------------
   pub(crate) id: WidgetId,
   pub(crate) geometry: Geometry,
   pub(crate) background_color: Rgba,
   //--------------------
   runtime: Option<Runtime>,
   state_flags: Cell<StateFlags>,
   number_mouse_buttons_pressed: Cell<i8>,
   //--------------------
   children_busy: Cell<WidgetId>,
   children: RefCell<ChildrenVec>,
   //--------------------
}

impl Internal {
   pub(crate) fn new<T>() -> Self {
      Self {
         parent: None,
         //--------------------
         id: WidgetId::new::<T>(),
         geometry: Geometry::default(),
         background_color: Rgba::GRAY,
         //--------------------
         runtime: None,
         state_flags: Cell::new(StateFlags::INIT),
         number_mouse_buttons_pressed: Cell::new(0),
         //--------------------
         children_busy: Cell::new(WidgetId::INVALID),
         children: Default::default(),
      }
   }
}

impl Internal {
   pub(crate) fn request_draw(&self) {
      let mut f = self.state_flags.get();

      if !f.contains(StateFlags::SELF_DRAW) {
         f.set(StateFlags::SELF_DRAW, true);
         self.state_flags.set(f);

         if let Some(parent) = &self.parent {
            if let Some(p) = parent.upgrade() {
               let mut bor = p.borrow_mut();
               let internal = bor.internal_mut();
               internal.state_flags.get_mut().set(StateFlags::CHILDREN_DRAW, true);
            }
         }
      }
   }

   pub(crate) fn request_update(&self) {
      let mut f = self.state_flags.get();

      if !f.contains(StateFlags::SELF_UPDATE) {
         f.set(StateFlags::SELF_UPDATE, true);
         self.state_flags.set(f);

         if let Some(parent) = &self.parent {
            if let Some(p) = parent.upgrade() {
               let mut bor = p.borrow_mut();
               let internal = bor.internal_mut();
               internal.state_flags.get_mut().set(StateFlags::CHILDREN_UPDATE, true);
            }
         }
      }
   }

   pub(crate) fn request_delete(&self) {
      let mut f = self.state_flags.get();

      if !f.contains(StateFlags::SELF_DELETE) {
         f.set(StateFlags::SELF_DELETE, true);
         self.state_flags.set(f);

         if let Some(parent) = &self.parent {
            if let Some(p) = parent.upgrade() {
               let mut bor = p.borrow_mut();
               let internal = bor.internal_mut();
               internal.state_flags.get_mut().set(StateFlags::CHILDREN_DELETE, true);
            }
         }
      }
   }
}

impl Internal {
   pub(crate) fn is_visible(&self) -> bool {
      self.state_flags.get().contains(StateFlags::IS_VISIBLE)
   }

   pub(crate) fn set_visible(&self, state: bool) -> bool {
      let mut f = self.state_flags.get();
      let curr = f.contains(StateFlags::IS_VISIBLE);
      if curr != state {
         f.set(StateFlags::IS_VISIBLE, state);
         self.state_flags.set(f);
         true
      } else {
         false
      }
   }

   pub(crate) fn is_enabled(&self) -> bool {
      self.state_flags.get().contains(StateFlags::IS_ENABLED)
   }

   pub(crate) fn set_enabled(&self, state: bool) -> bool {
      let mut f = self.state_flags.get();
      let curr = f.contains(StateFlags::IS_ENABLED);
      if curr != state {
         f.set(StateFlags::IS_ENABLED, state);
         self.state_flags.set(f);
         true
      } else {
         false
      }
   }

   pub(crate) fn is_transparent(&self) -> bool {
      self.state_flags.get().contains(StateFlags::IS_TRANSPARENT)
   }

   pub(crate) fn set_transparent(&self, state: bool) {
      let mut f = self.state_flags.get();
      f.set(StateFlags::IS_TRANSPARENT, state);
      self.state_flags.set(f);
   }

   pub(crate) fn is_over(&self) -> bool {
      self.state_flags.get().contains(StateFlags::IS_OVER)
   }

   pub(crate) fn set_over(&self, state: bool) {
      let mut f = self.state_flags.get();
      f.set(StateFlags::IS_OVER, state);
      self.state_flags.set(f);
   }

   pub(crate) fn has_mouse_tracking(&self) -> bool {
      self.state_flags.get().contains(StateFlags::HAS_MOUSE_TRACKING)
   }

   pub(crate) fn set_mouse_tracking(&self, state: bool) {
      let mut f = self.state_flags.get();
      f.set(StateFlags::HAS_MOUSE_TRACKING, state);
      self.state_flags.set(f);
   }

   pub(crate) fn add_mouse_btn_num(&self, num: i8) {
      let res = self.mouse_btn_num() + num;
      debug_assert!(res > -1, "inconsistent add/remove mouse buttons press in {:?}", self.id);
      self.number_mouse_buttons_pressed.set(res.max(0));
   }

   pub(crate) fn mouse_btn_num(&self) -> i8 {
      self.number_mouse_buttons_pressed.get()
   }
}

impl Internal {
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
   pub(crate) fn set_children(&mut self, mut ch: ChildrenVec, id: WidgetId) {
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

      #[cfg(debug_assertions)]
      {
         if let Some(p) = &c.parent {
            debug_assert!(p.upgrade().is_none(), "need re-parent implementation");
         }
      }

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
      dbg!(std::mem::size_of::<Internal>());
   }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
