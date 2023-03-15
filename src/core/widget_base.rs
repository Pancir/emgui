mod dispatcher;
mod focus;
mod runtime;

pub use dispatcher::*;

////////////////////////////////////////////////////////////////////////////////////////////////////

use crate::core::widget_base::runtime::Runtime;
use crate::core::{Geometry, IWidget, WidgetId};
use crate::defines::{DEFAULT_DOUBLE_CLICK_TIME, DEFAULT_TOOL_TIP_TIME, STATIC_CHILD_NUM};
use bitflags::bitflags;
use smallvec::SmallVec;
use std::cell::{Cell, RefCell};
use std::rc::{Rc, Weak};
use std::time::Duration;

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

pub struct WidgetBase {
   parent: Option<Weak<RefCell<dyn IWidget>>>,
   //--------------------
   id: WidgetId,
   geometry: Geometry,
   //--------------------
   tool_type_time: Option<Duration>,
   double_click_time: Option<Duration>,
   //--------------------
   runtime: Option<Runtime>,
   //--------------------
   state_flags: Cell<StateFlags>,
   number_mouse_buttons_pressed: Cell<i8>,
   //--------------------
   children_busy: Cell<WidgetId>,
   children: RefCell<ChildrenVec>,
   //--------------------
}

impl WidgetBase {
   pub fn new<T>() -> Self {
      Self {
         parent: None,
         //--------------------
         id: WidgetId::new::<T>(),
         geometry: Geometry::default(),
         //--------------------
         tool_type_time: None,
         double_click_time: None,
         //--------------------
         runtime: None,
         state_flags: Cell::new(StateFlags::INIT),
         number_mouse_buttons_pressed: Cell::new(0),
         //--------------------
         children_busy: Cell::new(WidgetId::INVALID),
         children: Default::default(),
      }
   }

   /// Unique id of the widget.
   pub fn id(&self) -> WidgetId {
      self.id
   }

   pub fn geometry(&self) -> &Geometry {
      &self.geometry
   }

   pub fn geometry_mut(&mut self) -> &mut Geometry {
      &mut self.geometry
   }
}

impl WidgetBase {
   /// Request redraw event.
   pub fn request_draw(&self) {
      let mut f = self.state_flags.get();

      if !f.contains(StateFlags::SELF_DRAW) {
         f.set(StateFlags::SELF_DRAW, true);
         self.state_flags.set(f);

         if let Some(parent) = &self.parent {
            if let Some(p) = parent.upgrade() {
               let mut bor = p.borrow_mut();
               let internal = bor.base_mut();
               internal.state_flags.get_mut().set(StateFlags::CHILDREN_DRAW, true);
            }
         }
      }
   }

   /// Request update event.
   pub fn request_update(&self) {
      let mut f = self.state_flags.get();

      if !f.contains(StateFlags::SELF_UPDATE) {
         f.set(StateFlags::SELF_UPDATE, true);
         self.state_flags.set(f);

         if let Some(parent) = &self.parent {
            if let Some(p) = parent.upgrade() {
               let mut bor = p.borrow_mut();
               let internal = bor.base_mut();
               internal.state_flags.get_mut().set(StateFlags::CHILDREN_UPDATE, true);
            }
         }
      }
   }

   /// Request to delete the widget.
   ///
   /// It schedules widget to delete the library will choose time to do it so,
   /// it will not be deleted immediately.
   pub fn request_delete(&self) {
      let mut f = self.state_flags.get();

      if !f.contains(StateFlags::SELF_DELETE) {
         f.set(StateFlags::SELF_DELETE, true);
         self.state_flags.set(f);

         if let Some(parent) = &self.parent {
            if let Some(p) = parent.upgrade() {
               let mut bor = p.borrow_mut();
               let internal = bor.base_mut();
               internal.state_flags.get_mut().set(StateFlags::CHILDREN_DELETE, true);
            }
         }
      }
   }
}

impl WidgetBase {
   /// Check if widget is visible.
   pub fn is_visible(&self) -> bool {
      self.state_flags.get().contains(StateFlags::IS_VISIBLE)
   }

   /// Set widget visible state.
   ///
   /// # Return
   /// `true` - if values was changed otherwise false.
   pub fn set_visible(&self, state: bool) -> bool {
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

   /// Check if widget is enabled.
   ///
   /// Disabled widget does not receive mouse ans keyboard inputs.
   pub fn is_enabled(&self) -> bool {
      self.state_flags.get().contains(StateFlags::IS_ENABLED)
   }

   /// Set widget enabled state.
   ///
   /// # Return
   /// `true` - if values was changed otherwise false.
   pub fn set_enabled(&self, state: bool) -> bool {
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

   /// Check if widget has transparent pixels.
   pub fn is_transparent(&self) -> bool {
      self.state_flags.get().contains(StateFlags::IS_TRANSPARENT)
   }

   /// Set if widget has transparent pixels.
   ///
   /// The draw engine should know if there are transparent pixels for
   /// selecting necessary algorithm to draw.
   pub fn set_transparent(&self, state: bool) {
      let mut f = self.state_flags.get();
      f.set(StateFlags::IS_TRANSPARENT, state);
      self.state_flags.set(f);
   }

   /// Check if mouse is over the widget's rectangle geometry.
   pub fn is_over(&self) -> bool {
      self.state_flags.get().contains(StateFlags::IS_OVER)
   }

   /// Set if mouse is over the widget's rectangle geometry.
   pub(crate) fn set_over(&self, state: bool) {
      let mut f = self.state_flags.get();
      f.set(StateFlags::IS_OVER, state);
      self.state_flags.set(f);
   }

   /// Check if the widget wants mouse tracking.
   ///
   /// See [Self::set_mouse_tracking]
   pub fn has_mouse_tracking(&self) -> bool {
      self.state_flags.get().contains(StateFlags::HAS_MOUSE_TRACKING)
   }

   /// Set if the widget wants mouse tracking.
   ///
   /// If mouse tracking is disabled (the default), the widget only receives
   /// mouse move events when at least one mouse button is pressed while the mouse is being moved.
   /// If mouse tracking is enabled, the widget receives mouse move events even
   /// if **NO** buttons are pressed.
   pub fn set_mouse_tracking(&self, state: bool) {
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

impl WidgetBase {
   /// Set waiting time for tooltip showing.
   ///
   /// If it is `None` the default is used.
   pub fn set_tool_type_time(&mut self, duration: Option<Duration>) {
      self.tool_type_time = duration;
   }

   /// Waiting time for tooltip showing.
   pub fn tool_type_time(&self) -> Duration {
      if let Some(v) = self.tool_type_time {
         v
      } else {
         if let Some(r) = &self.runtime {
            r.tool_type_time()
         } else {
            DEFAULT_TOOL_TIP_TIME
         }
      }
   }

   /// Set waiting time for double click detection.
   ///
   /// If it is `None` the default is used.
   pub fn set_double_click_time(&mut self, duration: Option<Duration>) {
      self.double_click_time = duration;
   }

   /// Waiting time for double click detection.
   pub fn double_click_time(&self) -> Duration {
      if let Some(v) = self.double_click_time {
         v
      } else {
         if let Some(r) = &self.runtime {
            r.double_click_time()
         } else {
            DEFAULT_DOUBLE_CLICK_TIME
         }
      }
   }
}

impl WidgetBase {
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
      let pch = bor.base();

      debug_assert!(!pch.children_busy.get().is_valid());

      let mut bor_ch = pch.children.borrow_mut();
      bor_ch.push(child.clone());
   }

   {
      let mut bor = child.borrow_mut();
      let c = bor.base_mut();

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
      dbg!(std::mem::size_of::<WidgetBase>());
   }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
