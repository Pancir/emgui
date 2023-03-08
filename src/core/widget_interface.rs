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

   /// Get the widget type name for debugging purposes.
   /// Developers should not override this method.
   fn type_name(&self) -> &'static str {
      std::any::type_name::<Self>()
   }

   /// Get the widget type name for debugging purposes.
   ///
   /// Developers should not override this method.
   fn type_name_short(&self) -> &'static str {
      let name = self.type_name();
      name.split('<').next().unwrap_or(name).split("::").last().unwrap_or(name)
   }

   fn id(&self) -> WidgetId;

   //---------------------------------------

   fn request_draw(&self);
   fn request_delete(&self);
   fn request_update(&self);
   fn request_focus(&self);

   //---------------------------------------

   fn base(&self) -> &WidgetBase;
   fn base_mut(&mut self) -> &mut WidgetBase;

   //---------------------------------------

   fn set_tool_type_time(&mut self, duration: Option<Duration>);
   fn tool_type_time(&self) -> Duration;

   fn set_double_click_time(&mut self, duration: Option<Duration>);
   fn double_click_time(&self) -> Duration;

   //---------------------------------------

   fn derive(&self) -> &dyn Derive;
   fn derive_mut(&mut self) -> &mut dyn Derive;

   fn geometry(&self) -> &Geometry;

   fn set_rect(&mut self, r: Rect<f32>);

   //---------------------------------------

   fn is_visible(&self) -> bool;
   fn set_visible(&mut self, state: bool);

   fn is_enabled(&self) -> bool;
   fn set_enabled(&mut self, state: bool);

   fn is_transparent(&self) -> bool;
   fn set_transparent(&mut self, state: bool);

   //---------------------------------------

   ///This property holds whether mouse tracking is enabled for the widget.
   /// If mouse tracking is disabled (the default), the widget only receives
   /// mouse move events when at least one mouse button is pressed while the mouse is being moved.
   /// If mouse tracking is enabled, the widget receives mouse move events even if no buttons are pressed.
   ///
   /// See [Self::has_mouse_tracking]
   fn set_mouse_tracking(&mut self, state: bool);

   /// See [Self::set_mouse_tracking]
   fn has_mouse_tracking(&mut self) -> bool;

   /// `True` if mouse is over the widget.
   fn is_mouse_over(&self) -> bool;

   //---------------------------------------

   fn emit_lifecycle(&mut self, _event: &LifecycleEventCtx);
   fn emit_layout(&mut self, _event: &LayoutEventCtx);
   fn emit_draw(&mut self, _canvas: &mut Canvas, event: &DrawEventCtx);
   fn emit_update(&mut self, _event: &UpdateEventCtx);
   fn emit_mouse_enter(&mut self);
   fn emit_mouse_leave(&mut self);
   fn emit_mouse_move(&mut self, _event: &MouseMoveEventCtx) -> bool;
   fn emit_mouse_button(&mut self, _event: &MouseButtonsEventCtx) -> bool;
   fn emit_mouse_wheel(&mut self, _event: &MouseWheelEventCtx) -> bool;
   fn emit_keyboard(&mut self, _event: &KeyboardEventCtx) -> bool;
}

////////////////////////////////////////////////////////////////////////////////////////////////////
