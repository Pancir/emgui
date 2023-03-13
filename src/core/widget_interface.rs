use crate::core::derive::Derive;
use crate::core::events::{
   DrawEventCtx, KeyboardEventCtx, LayoutEventCtx, LifecycleEventCtx, MouseButtonsEventCtx,
   MouseMoveEventCtx, MouseWheelEventCtx, UpdateEventCtx,
};
use crate::core::widget_base::WidgetBase;
use crate::core::{Geometry, WidgetId};
use sim_draw::m::Rect;
use sim_draw::Canvas;
use std::any::Any;
use std::time::Duration;

////////////////////////////////////////////////////////////////////////////////////////////////////

pub trait IWidget: Any + 'static {
   fn as_any(&self) -> &dyn Any;
   fn as_any_mut(&mut self) -> &mut dyn Any;

   //---------------------------------------

   /// Get the widget type name for debugging purposes.
   /// Developers should not override this method.
   #[inline]
   fn type_name(&self) -> &'static str {
      std::any::type_name::<Self>()
   }

   /// Get the widget type name for debugging purposes.
   ///
   /// Developers should not override this method.
   #[inline]
   fn type_name_short(&self) -> &'static str {
      let name = self.type_name();
      name.split('<').next().unwrap_or(name).split("::").last().unwrap_or(name)
   }

   #[inline]
   fn id(&self) -> WidgetId {
      self.base().id()
   }

   //---------------------------------------

   fn base(&self) -> &WidgetBase;
   fn base_mut(&mut self) -> &mut WidgetBase;

   fn derive(&self) -> &dyn Derive;
   fn derive_mut(&mut self) -> &mut dyn Derive;

   //---------------------------------------

   #[cfg_attr(feature = "trace-widget",
   tracing::instrument(skip(self), fields(WidgetID = self.id().raw())))]
   fn request_draw(&self) {
      self.base().request_draw();
   }

   #[cfg_attr(feature = "trace-widget",
   tracing::instrument(skip(self), fields(WidgetID = self.id().raw())))]
   fn request_delete(&self) {
      self.base().request_delete();
   }

   #[cfg_attr(feature = "trace-widget",
   tracing::instrument(skip(self), fields(WidgetID = self.id().raw())))]
   fn request_update(&self) {
      self.base().request_update();
   }

   #[cfg_attr(feature = "trace-widget",
   tracing::instrument(skip(self), fields(WidgetID = self.id().raw())))]
   fn request_focus(&self) {
      unimplemented!()
   }

   //---------------------------------------

   #[inline]
   fn set_tool_type_time(&mut self, duration: Option<Duration>) {
      self.base_mut().set_tool_type_time(duration);
   }

   #[inline]
   fn tool_type_time(&self) -> Duration {
      self.base().tool_type_time()
   }

   #[inline]
   fn set_double_click_time(&mut self, duration: Option<Duration>) {
      self.base_mut().set_double_click_time(duration);
   }

   #[inline]
   fn double_click_time(&self) -> Duration {
      self.base().double_click_time()
   }

   //---------------------------------------

   #[inline]
   fn geometry(&self) -> &Geometry {
      self.base().geometry()
   }

   fn set_rect(&mut self, r: Rect<f32>);

   //---------------------------------------

   #[inline]
   fn is_visible(&self) -> bool {
      self.base().is_visible()
   }

   fn set_visible(&mut self, state: bool);

   #[inline]
   fn is_enabled(&self) -> bool {
      self.base().is_enabled()
   }

   fn set_enabled(&mut self, state: bool);

   #[inline]
   fn is_transparent(&self) -> bool {
      self.base().is_transparent()
   }

   fn set_transparent(&mut self, state: bool);

   //---------------------------------------

   ///This property holds whether mouse tracking is enabled for the widget.
   /// If mouse tracking is disabled (the default), the widget only receives
   /// mouse move events when at least one mouse button is pressed while the mouse is being moved.
   /// If mouse tracking is enabled, the widget receives mouse move events even if no buttons are pressed.
   ///
   /// See [Self::has_mouse_tracking]
   #[inline]
   fn set_mouse_tracking(&mut self, state: bool) {
      self.base_mut().set_mouse_tracking(state);
   }

   /// See [Self::set_mouse_tracking]
   #[inline]
   fn has_mouse_tracking(&mut self) -> bool {
      self.base().has_mouse_tracking()
   }

   /// `True` if mouse is over the widget.
   #[inline]
   fn is_mouse_over(&self) -> bool {
      self.base().is_over()
   }

   //---------------------------------------

   fn emit_lifecycle(&mut self, _event: &LifecycleEventCtx);
   fn emit_layout(&mut self, _event: &LayoutEventCtx);
   fn emit_draw(&mut self, _canvas: &mut Canvas, _event: &DrawEventCtx);
   fn emit_update(&mut self, _event: &UpdateEventCtx);
   fn emit_mouse_enter(&mut self);
   fn emit_mouse_leave(&mut self);
   fn emit_mouse_move(&mut self, _event: &MouseMoveEventCtx) -> bool;
   fn emit_mouse_button(&mut self, _event: &MouseButtonsEventCtx) -> bool;
   fn emit_mouse_wheel(&mut self, _event: &MouseWheelEventCtx) -> bool;
   fn emit_keyboard(&mut self, _event: &KeyboardEventCtx) -> bool;
}

////////////////////////////////////////////////////////////////////////////////////////////////////
