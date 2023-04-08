use crate::core::derive::Derive;
use crate::core::events::{
   DrawEventCtx, KeyboardEventCtx, LayoutEventCtx, LifecycleEventCtx, MouseButtonsEventCtx,
   MouseMoveEventCtx, MouseWheelEventCtx, UpdateEventCtx,
};
use crate::core::widget_base::WidgetBase;
use sim_draw::m::Rect;
use sim_draw::Canvas;
use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use super::events::LifecycleState;

////////////////////////////////////////////////////////////////////////////////////////////////////

/// Common interface for all widgets.
///
/// See the [crate::WidgetBase] as well which is part of it.
pub trait IWidget: Any + 'static {
   fn as_any(&self) -> &dyn Any;
   fn as_any_mut(&mut self) -> &mut dyn Any;

   /// Create `Rc`.
   ///
   /// TODO maybe Pin?
   fn to_rc(self) -> Rc<RefCell<Self>>
   where
      Self: Sized,
   {
      let s = Rc::new(RefCell::new(self));
      let w = Rc::downgrade(&s);

      match s.try_borrow_mut() {
         Ok(mut widget) => {
            let event = LifecycleEventCtx { state: LifecycleState::SelfReference(w) };
            widget.emit_lifecycle(&event);
         }
         Err(_) => {
            // # Safety
            // The widget is just created and owned by this function.
            unsafe { std::hint::unreachable_unchecked() };
         }
      }
      s
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

   fn base(&self) -> &WidgetBase;
   fn base_mut(&mut self) -> &mut WidgetBase;

   fn derive(&self) -> &dyn Derive;
   fn derive_mut(&mut self) -> &mut dyn Derive;

   //---------------------------------------

   fn set_rect(&mut self, r: Rect<f32>);

   //---------------------------------------

   fn set_visible(&mut self, state: bool);
   fn set_enabled(&mut self, state: bool);
   fn set_transparent(&mut self, state: bool);

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
