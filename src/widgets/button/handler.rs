use super::ButtonState;
use crate::core::input::mouse::MouseButton;

////////////////////////////////////////////////////////////////////////////////////////////////////

pub trait IButtonHandler {
   /// This is called when the button is activated.
   ///
   /// (i.e., pressed down then released while the mouse cursor is inside the button).
   ///
   /// The [ButtonState::is_toggle] is the current toggle state.
   fn click(&mut self, _state: &ButtonState) {}

   /// This is called when the button is pressed down.
   ///
   /// The [ButtonState::is_down] is `true`.
   fn pressed(&mut self, _state: &ButtonState, _button: MouseButton) {}

   /// This is called when the button is released.
   ///
   /// The [ButtonState::is_down] is `false`.
   fn released(&mut self, _state: &ButtonState, _button: MouseButton) {}
}

////////////////////////////////////////////////////////////////////////////////////////////////////

/// Default button handler.
///
/// This implementation uses closures allocated in heap so,
/// in some cases it is better to create you own.
///
/// # Note
/// Heap allocation happens only when you add a closure.
#[derive(Default)]
#[allow(clippy::type_complexity)]
pub struct ButtonHandler {
   on_click: Option<Box<dyn FnMut(&ButtonState)>>,
   on_pressed: Option<Box<dyn FnMut(&ButtonState, MouseButton)>>,
   on_released: Option<Box<dyn FnMut(&ButtonState, MouseButton)>>,
}

impl ButtonHandler {
   /// Construct new.
   pub fn new() -> Self {
      Self::default()
   }

   /// Set callback.
   ///
   /// It allocates memory in heap for the closure.
   pub fn on_click(mut self, cb: impl FnMut(&ButtonState) + 'static) -> Self {
      self.on_click = Some(Box::new(cb));
      self
   }

   /// Set callback.
   ///
   /// It allocates memory in heap for the closure.
   pub fn on_pressed(mut self, cb: impl FnMut(&ButtonState, MouseButton) + 'static) -> Self {
      self.on_pressed = Some(Box::new(cb));
      self
   }

   /// Set callback.
   ///
   /// It allocates memory in heap for the closure.
   pub fn on_released(mut self, cb: impl FnMut(&ButtonState, MouseButton) + 'static) -> Self {
      self.on_released = Some(Box::new(cb));
      self
   }
}

impl IButtonHandler for ButtonHandler {
   fn click(&mut self, state: &ButtonState) {
      if let Some(h) = &mut self.on_click {
         (h)(state)
      }
   }

   fn pressed(&mut self, state: &ButtonState, mb: MouseButton) {
      if let Some(h) = &mut self.on_pressed {
         (h)(state, mb)
      }
   }

   fn released(&mut self, state: &ButtonState, mb: MouseButton) {
      if let Some(h) = &mut self.on_released {
         (h)(state, mb)
      }
   }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
