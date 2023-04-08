use crate::core::derive::Derive;
use crate::core::events::{
   DrawEventCtx, KeyboardEventCtx, LayoutEventCtx, LifecycleEventCtx, LifecycleState,
   MouseButtonsEventCtx, MouseMoveEventCtx, MouseWheelEventCtx, UpdateEventCtx,
};
use crate::core::{IWidget, WidgetBase};
use sim_draw::color::Rgba;
use sim_draw::m::Rect;
use sim_draw::{Canvas, Paint};
use std::any::Any;
use std::cell::RefCell;
use std::mem::MaybeUninit;
use std::rc::Rc;

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone)]
pub struct WidgetVt<W> {
   /// It is called when a new rectangle has to be set.
   ///
   /// # Return
   /// `Some` - rect to set. `None` - means there is no rect to set.
   pub on_set_rect: fn(w: &mut W, Rect<f32>) -> Option<Rect<f32>>,
   //-------------------------------------------------
   /// It is called when visible state is changed.
   pub on_visible: fn(w: &mut W, bool),
   /// It is called when disable state is changed.
   pub on_disable: fn(w: &mut W, bool),
   //-------------------------------------------------
   pub on_lifecycle: fn(w: &mut W, &LifecycleEventCtx),
   pub on_layout: fn(w: &mut W, &LayoutEventCtx),
   //-------------------------------------------------
   /// It is called when there is a request to update.
   pub on_update: fn(w: &mut W, &UpdateEventCtx),
   /// It is called whenever the widget must be re-drawn,
   pub on_draw: fn(w: &mut W, &mut Canvas, &DrawEventCtx),
   //-------------------------------------------------
   /// It is called when the mouse cursor enters the widget.
   pub on_mouse_enter: fn(w: &mut W),
   /// It is called when the mouse cursor leaves the widget.
   pub on_mouse_leave: fn(w: &mut W),
   //-------------------------------------------------
   /// It is called when the mouse cursor is moved inside the widget rectangle.
   ///
   /// If mouse tracking is switched off, mouse move events only occur if
   /// a mouse button is pressed while the mouse is being moved.
   /// If [mouse tracking](IWidget::set_mouse_tracking) is switched on,
   /// mouse move events occur even if **NO** mouse button is pressed.
   pub on_mouse_move: fn(w: &mut W, &MouseMoveEventCtx) -> bool,
   pub on_mouse_button: fn(w: &mut W, &MouseButtonsEventCtx) -> bool,
   pub on_mouse_wheel: fn(w: &mut W, &MouseWheelEventCtx) -> bool,

   pub on_keyboard: fn(w: &mut W, &KeyboardEventCtx) -> bool,
}

impl<W> Default for WidgetVt<W> {
   /// Create default where all the function do nothing.
   fn default() -> Self {
      Self {
         on_set_rect: |_, v| Some(v),
         //--------------------------------------
         on_visible: |_, _| {},
         on_disable: |_, _| {},
         //--------------------------------------
         on_lifecycle: |_, _| {},
         on_layout: |_, _| {},
         //--------------------------------------
         on_update: |_, _| {},
         on_draw: |_, _, _| {},
         //--------------------------------------
         on_mouse_enter: |_| {},
         on_mouse_leave: |_| {},
         //--------------------------------------
         on_mouse_move: |_, _| true,
         on_mouse_button: |_, _| true,
         on_mouse_wheel: |_, _| true,
         //--------------------------------------
         on_keyboard: |_, _| false,
      }
   }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct Widget<D>
where
   D: Derive,
{
   inherited: MaybeUninit<D>,
   vtable: WidgetVt<Self>,
   base: WidgetBase,
   background_color: Rgba,
}

impl Default for Widget<()> {
   fn default() -> Self {
      Self::new()
   }
}

impl Widget<()> {
   /// Construct new.
   pub fn new() -> Self {
      Self::inherit(|_| (), |_| {})
   }
}

impl<D: 'static> Widget<D>
where
   D: Derive,
{
   /// Construct new and init.
   pub fn inherit<CB1, CB2>(vt_cb: CB1, init_cb: CB2) -> Self
   where
      CB1: FnOnce(&mut WidgetVt<Self>) -> D,
      CB2: FnOnce(&mut Self),
   {
      let mut out = Self {
         inherited: unsafe { std::mem::zeroed() },
         background_color: Rgba::GRAY,
         vtable: WidgetVt {
            on_draw: Self::on_draw,
            //--------------------------------------
            // TODO remove draw as it is for testing
            on_mouse_enter: |w| w.base().request_draw(),
            on_mouse_leave: |w| w.base().request_draw(),
            //--------------------------------------
            ..WidgetVt::default()
         },
         base: WidgetBase::new::<Self>(),
      };

      let inherited = vt_cb(&mut out.vtable);
      out.inherited.write(inherited);

      init_cb(&mut out);
      out
   }

   //---------------------------------------

   #[inline]
   pub fn inherited_obj(&self) -> &D {
      // # Safety
      // All initialization happen in a constructor.
      unsafe { self.inherited.assume_init_ref() }
   }

   #[inline]
   pub fn inherited_obj_mut(&mut self) -> &mut D {
      // # Safety
      // All initialization happen in a constructor.
      unsafe { self.inherited.assume_init_mut() }
   }

   pub fn set_background_color(&mut self, color: Rgba) {
      debug_assert!(color.a > (0.0 - f32::EPSILON) && color.a < 1.0 + f32::EPSILON);
      self.background_color = color;
      self.set_transparent(color.a < 1.0);
   }

   pub fn background_color(&mut self) -> Rgba {
      self.background_color
   }
}

impl<D: 'static> IWidget for Widget<D>
where
   D: Derive,
{
   fn as_any(&self) -> &dyn Any {
      self
   }

   fn as_any_mut(&mut self) -> &mut dyn Any {
      self
   }

   //---------------------------------------

   fn base(&self) -> &WidgetBase {
      &self.base
   }

   fn base_mut(&mut self) -> &mut WidgetBase {
      &mut self.base
   }

   fn inherited(&self) -> &dyn Derive {
      self.inherited_obj()
   }

   fn inherited_mut(&mut self) -> &mut dyn Derive {
      self.inherited_obj_mut()
   }

   //---------------------------------------

   fn set_rect(&mut self, r: Rect<f32>) {
      if let Some(rect) = (self.vtable.on_set_rect)(self, r) {
         self.base.geometry_mut().set_rect(rect);
      }
   }

   //---------------------------------------

   fn set_visible(&mut self, state: bool) {
      if self.base.set_visible(state) {
         (self.vtable.on_visible)(self, state)
      }
   }

   fn set_enabled(&mut self, state: bool) {
      if self.base.set_enabled(state) {
         (self.vtable.on_disable)(self, !state)
      }
   }

   fn set_transparent(&mut self, state: bool) {
      self.base.set_transparent(state)
   }

   //---------------------------------------

   #[cfg_attr(feature = "trace-widget",
   tracing::instrument(skip(self, event), fields(WidgetID = self.base().id().raw(), ret)))]
   fn emit_lifecycle(&mut self, event: &LifecycleEventCtx) {
      (self.vtable.on_lifecycle)(self, event);
   }

   #[cfg_attr(feature = "trace-widget",
   tracing::instrument(skip(self, event), fields(WidgetID = self.base().id().raw(), ret)))]
   fn emit_layout(&mut self, event: &LayoutEventCtx) {
      (self.vtable.on_layout)(self, event);
   }

   #[cfg_attr(feature = "trace-widget",
   tracing::instrument(skip(self, canvas, event), fields(WidgetID = self.base().id().raw(), ret)))]
   fn emit_draw(&mut self, canvas: &mut Canvas, event: &DrawEventCtx) {
      (self.vtable.on_draw)(self, canvas, event);
   }

   #[cfg_attr(feature = "trace-widget",
   tracing::instrument(skip(self, event), fields(WidgetID = self.base().id().raw(), ret)))]
   fn emit_update(&mut self, event: &UpdateEventCtx) {
      (self.vtable.on_update)(self, event);
   }

   #[cfg_attr(feature = "trace-widget",
   tracing::instrument(skip(self), fields(WidgetID = self.base().id().raw()), ret))]
   fn emit_mouse_enter(&mut self) {
      (self.vtable.on_mouse_enter)(self)
   }

   #[cfg_attr(feature = "trace-widget",
   tracing::instrument(skip(self), fields(WidgetID = self.base().id().raw()), ret))]
   fn emit_mouse_leave(&mut self) {
      (self.vtable.on_mouse_leave)(self)
   }

   #[must_use]
   #[cfg_attr(feature = "trace-widget",
   tracing::instrument(skip(self, event), fields(WidgetID = self.base().id().raw()), ret))]
   fn emit_mouse_move(&mut self, event: &MouseMoveEventCtx) -> bool {
      (self.vtable.on_mouse_move)(self, event)
   }

   #[must_use]
   #[cfg_attr(feature = "trace-widget",
   tracing::instrument(skip(self, event), fields(WidgetID = self.base().id().raw()), ret))]
   fn emit_mouse_button(&mut self, event: &MouseButtonsEventCtx) -> bool {
      (self.vtable.on_mouse_button)(self, event)
   }

   #[must_use]
   #[cfg_attr(feature = "trace-widget",
   tracing::instrument(skip(self, event), fields(WidgetID = self.base().id().raw()), ret))]
   fn emit_mouse_wheel(&mut self, event: &MouseWheelEventCtx) -> bool {
      (self.vtable.on_mouse_wheel)(self, event)
   }

   #[must_use]
   #[cfg_attr(feature = "trace-widget",
   tracing::instrument(skip(self, event), fields(WidgetID = self.base().id().raw()), ret))]
   fn emit_keyboard(&mut self, event: &KeyboardEventCtx) -> bool {
      (self.vtable.on_keyboard)(self, event)
   }
}

impl<D: 'static> Widget<D>
where
   D: Derive,
{
   fn on_draw(&mut self, canvas: &mut Canvas, _event: &DrawEventCtx) {
      // TODO remove over as it is for testing
      if self.base.is_over() {
         canvas.set_paint(Paint::new_color(Rgba::RED.with_alpha_mul(0.2)));
      } else {
         canvas.set_paint(Paint::new_color(self.background_color()));
      }
      canvas.fill(&self.base.geometry().rect());
   }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
   use super::*;

   #[test]
   fn sizes() {
      dbg!(std::mem::size_of::<WidgetVt<()>>());
      dbg!(std::mem::size_of::<Widget<()>>());
   }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
