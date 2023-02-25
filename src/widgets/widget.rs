use crate::elements::BaseState;
use crate::widgets::events::{
   KeyboardEvent, LayoutEvent, LifecycleEvent, MouseButtonsEvent, MouseMoveEvent, MouseWheelEvent,
   UpdateEvent,
};
use sim_draw::m::Rect;
use sim_draw::Canvas;
use std::any::Any;
use std::rc::Weak;

////////////////////////////////////////////////////////////////////////////////////////////////////

pub trait IWidget: Any + 'static {
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

   //---------------------------------------

   /// Access to base widget data.
   fn base_state(&self) -> &BaseState;

   fn set_parent(&mut self, parent: Option<Weak<dyn IWidget>>);

   /// Access to base widget data.
   fn parent(&self) -> &Option<Weak<dyn IWidget>>;

   /// Set new rectangle to the widget.
   fn set_rect(&mut self, rect: Rect<f32>);

   /// Request draw event.
   fn request_draw(&self) {
      self.base_state().needs_draw.set(true);
      if let Some(p) = self.parent() {
         if let Some(o) = p.upgrade() {
            o.request_draw();
         }
      }
   }

   //---------------------------------------

   fn on_lifecycle(&mut self, _event: &mut LifecycleEvent) {}

   fn on_layout(&mut self, _event: &LayoutEvent) {}

   fn on_draw(&mut self, _canvas: &mut Canvas) {}

   fn on_update(&mut self, _event: &UpdateEvent) {}

   #[must_use]
   fn on_mouse_move(&mut self, _event: &MouseMoveEvent) -> bool {
      false
   }

   #[must_use]
   fn on_mouse_button(&mut self, _event: &MouseButtonsEvent) -> bool {
      false
   }

   #[must_use]
   fn on_mouse_wheel(&mut self, _event: &MouseWheelEvent) -> bool {
      false
   }

   #[must_use]
   fn on_keyboard(&mut self, _event: &KeyboardEvent) -> bool {
      false
   }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
