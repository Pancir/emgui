mod children;
mod dispatcher;
mod focus;
mod runtime;

use self::children::Children;
pub use dispatcher::*;
use sim_draw::m::Rect;

////////////////////////////////////////////////////////////////////////////////////////////////////

use super::{WidgetRef, WidgetRefOwner};
use crate::core::widget_base::runtime::Runtime;
use crate::core::{Geometry, WidgetId};
use crate::defines::{DEFAULT_DOUBLE_CLICK_TIME, DEFAULT_TOOL_TIP_TIME};
use bitflags::bitflags;
use std::cell::Cell;
use std::time::Duration;

////////////////////////////////////////////////////////////////////////////////////////////////////

bitflags! {
   /// Control flow for events.
   #[derive(Debug, Clone, Copy, PartialEq, Eq)]
   pub(crate) struct StateFlags: u16 {
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

      const INIT = Self::SELF_DRAW.bits()|Self::CHILDREN_DRAW.bits()|Self::SELF_UPDATE.bits()|Self::CHILDREN_UPDATE.bits()|Self::IS_VISIBLE.bits()|Self::IS_ENABLED.bits();
   }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct WidgetBase {
   parent: Option<WidgetRef>,
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
   children: Children,
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
         children: Children::default(),
      }
   }

   /// Unique id of the widget.
   #[inline]
   pub fn id(&self) -> WidgetId {
      self.id
   }

   #[inline]
   pub fn geometry(&self) -> &Geometry {
      &self.geometry
   }

   #[inline]
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
               let mut bor = p.widget_mut().unwrap();
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
               let mut bor = p.widget_mut().unwrap();
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
               let mut bor = p.widget_mut().unwrap();
               let internal = bor.base_mut();
               internal.state_flags.get_mut().set(StateFlags::CHILDREN_DELETE, true);
            }
         }
      }
   }
}

impl WidgetBase {
   /// Check if widget is visible.
   #[inline]
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
   #[inline]
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
   #[inline]
   pub fn is_transparent(&self) -> bool {
      self.state_flags.get().contains(StateFlags::IS_TRANSPARENT)
   }

   /// Set if widget has transparent pixels.
   ///
   /// The draw engine should know if there are transparent pixels for
   /// selecting necessary algorithm to draw.
   #[inline]
   pub fn set_transparent(&self, state: bool) {
      let mut f = self.state_flags.get();
      f.set(StateFlags::IS_TRANSPARENT, state);
      self.state_flags.set(f);
   }

   /// Check if mouse is over the widget's rectangle geometry.
   #[inline]
   pub fn is_over(&self) -> bool {
      self.state_flags.get().contains(StateFlags::IS_OVER)
   }

   /// Set if mouse is over the widget's rectangle geometry.
   #[inline]
   pub(crate) fn set_over(&self, state: bool) {
      let mut f = self.state_flags.get();
      f.set(StateFlags::IS_OVER, state);
      self.state_flags.set(f);
   }

   /// Check if the widget wants mouse tracking.
   ///
   /// See [Self::set_mouse_tracking]
   #[inline]
   pub fn has_mouse_tracking(&self) -> bool {
      self.state_flags.get().contains(StateFlags::HAS_MOUSE_TRACKING)
   }

   /// Set if the widget wants mouse tracking.
   ///
   /// If mouse tracking is disabled (the default), the widget only receives
   /// mouse move events when at least one mouse button is pressed while the mouse is being moved.
   /// If mouse tracking is enabled, the widget receives mouse move events even
   /// if **NO** buttons are pressed.
   #[inline]
   pub fn set_mouse_tracking(&self, state: bool) {
      let mut f = self.state_flags.get();
      f.set(StateFlags::HAS_MOUSE_TRACKING, state);
      self.state_flags.set(f);
   }

   #[inline]
   pub(crate) fn add_mouse_btn_num(&self, num: i8) {
      let res = self.mouse_btn_num() + num;
      debug_assert!(res > -1, "inconsistent add/remove mouse buttons press in {:?}", self.id);
      self.number_mouse_buttons_pressed.set(res.max(0));
   }

   #[inline]
   pub(crate) fn mouse_btn_num(&self) -> i8 {
      self.number_mouse_buttons_pressed.get()
   }

   #[inline]
   pub fn children(&self) -> &Children {
      &self.children
   }

   #[inline]
   pub fn children_mut(&mut self) -> &mut Children {
      &mut self.children
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
      } else if let Some(r) = &self.runtime {
         r.tool_type_time()
      } else {
         DEFAULT_TOOL_TIP_TIME
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
      } else if let Some(r) = &self.runtime {
         r.double_click_time()
      } else {
         DEFAULT_DOUBLE_CLICK_TIME
      }
   }
}

impl WidgetBase {
   #[inline]
   pub(crate) fn data_for_dispatcher(&self) -> (StateFlags, WidgetId, Rect<f32>, i8, bool) {
      (
         self.state_flags.get(),
         self.id,
         self.geometry.rect(),
         self.mouse_btn_num(),
         self.has_mouse_tracking(),
      )
   }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn add_children<const NUM: usize>(parent: &WidgetRefOwner, children: [WidgetRefOwner; NUM]) {
   // TODO swap implementation with add_child, it will be faster as we can only once borrow.

   for c in children {
      add_child(parent, c);
   }
}

pub fn add_child(parent: &WidgetRefOwner, child: WidgetRefOwner) -> WidgetRef {
   // TODO what about runtime propagation?

   let w = child.as_ref();
   {
      let bor = parent.widget().unwrap();
      let pch = bor.base();

      debug_assert!(!pch.children().is_under_process());

      let mut bor_ch = pch.children.access();
      bor_ch.push(child.clone());
   }

   {
      let mut bor = child.widget_mut().unwrap();
      let c = bor.base_mut();

      #[cfg(debug_assertions)]
      {
         if let Some(p) = &c.parent {
            debug_assert!(p.upgrade().is_none(), "need re-parent implementation");
         }
      }

      c.parent = Some(parent.as_ref());
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
