use crate::core::events::{
   DrawEventCtx, KeyboardEventCtx, LayoutEventCtx, LifecycleEventCtx, MouseButtonsEventCtx,
   MouseMoveEventCtx, MouseWheelEventCtx, UpdateEventCtx,
};
use sim_draw::m::Rect;
use sim_draw::Canvas;

////////////////////////////////////////////////////////////////////////////////////////////////////

/// Widget function table.
///
/// # Warning
///
/// * Be careful when you replace functions with your ones, it actually may have
///   some default implementation that should be invoked every time.
/// * Some of functions maybe totally ignored.
/// * see documentation of a widget this table belongs to for more information.
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
   /// It is called when the mouse cursor enters or leave the widget.
   ///
   /// # Arguments
   ///
   /// * `enter`: `true` if mouse enter otherwise `false`.
   pub on_mouse_cross: fn(w: &mut W, enter: bool),
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
         on_mouse_cross: |_, _| {},
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
