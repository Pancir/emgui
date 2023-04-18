use super::{Painter, WidgetStrongRef};
use crate::core::events::{
   DrawEventCtx, KeyboardEventCtx, LayoutEventCtx, LifecycleEventCtx, MouseButtonsEventCtx,
   MouseMoveEventCtx, MouseWheelEventCtx, UpdateEventCtx,
};
use crate::core::widget_base::WidgetBase;
use sim_draw::m::Rect;
use std::any::Any;

////////////////////////////////////////////////////////////////////////////////////////////////////

/// Common interface for all widgets.
///
/// See the [crate::WidgetBase] as well which is part of it.
pub trait IWidget: Any + 'static {
   fn as_any(&self) -> &dyn Any;
   fn as_any_mut(&mut self) -> &mut dyn Any;

   fn to_ref(self) -> WidgetStrongRef
   where
      Self: Sized,
   {
      WidgetStrongRef::new(self)
   }

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

   //---------------------------------------

   /// Notify widget.
   ///
   /// # Arguments
   /// * `message`: any message type.
   fn notify(&mut self, _message: &dyn Any) {}

   //---------------------------------------

   fn base(&self) -> &WidgetBase;
   fn base_mut(&mut self) -> &mut WidgetBase;

   fn inherited(&self) -> &dyn Any;
   fn inherited_mut(&mut self) -> &mut dyn Any;

   //---------------------------------------

   fn set_rect(&mut self, r: Rect<f32>);

   //---------------------------------------

   fn set_visible(&mut self, state: bool);
   fn set_enabled(&mut self, state: bool);
   fn set_transparent(&mut self, state: bool);

   //---------------------------------------

   fn on_lifecycle(&mut self, event: &LifecycleEventCtx);
   fn on_layout(&mut self, event: &LayoutEventCtx);
   fn on_draw(&mut self, canvas: &mut Painter, _event: &DrawEventCtx);
   fn on_update(&mut self, _event: &UpdateEventCtx);
   fn on_mouse_cross(&mut self, enter: bool);
   fn on_mouse_move(&mut self, event: &MouseMoveEventCtx) -> bool;
   fn on_mouse_button(&mut self, event: &MouseButtonsEventCtx) -> bool;
   fn on_mouse_wheel(&mut self, event: &MouseWheelEventCtx) -> bool;
   fn on_keyboard(&mut self, event: &KeyboardEventCtx) -> bool;
}

////////////////////////////////////////////////////////////////////////////////////////////////////
