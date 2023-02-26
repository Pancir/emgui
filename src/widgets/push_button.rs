use crate::core::derive::Derive;
use crate::core::events::{DrawEventCtx, MouseButtonsEventCtx, MouseMoveEventCtx};
use crate::core::{IWidget, Widget};
use crate::elements::Label;
use sim_draw::color::Rgba;
use sim_draw::m::Rect;
use sim_draw::{Canvas, Paint, TextAlign, TextPaint};
use sim_input::mouse::{MouseButton, MouseState};
use std::any::Any;
use std::borrow::Cow;
use std::cell::RefCell;
use std::rc::Rc;

////////////////////////////////////////////////////////////////////////////////////////////////////

pub trait IPushButtonHandler {
   /// This is called when the button is activated.
   ///
   /// (i.e., pressed down then released while the mouse cursor is inside the button).
   ///
   /// The [PushButtonState::is_toggle] is the current state of toggle.
   fn click(&mut self, _state: &PushButtonState) {}

   /// This is called when the button is pressed down.
   ///
   /// The [PushButtonState::is_down] is `true`.
   fn pressed(&mut self, _state: &PushButtonState, _button: MouseButton) {}

   /// This is called when the button is released.
   ///
   /// The [PushButtonState::is_down] is `false`.
   fn released(&mut self, _state: &PushButtonState, _button: MouseButton) {}
}

/// Default button handler.
///
/// This implementation uses closures allocated in heap so,
/// in some cases it is better to create you own.
///
/// # Note
/// Heap allocation happens only when you add a closure.
#[derive(Default)]
pub struct PushButtonHandler {
   on_click: Option<Box<dyn FnMut(&PushButtonState)>>,
   on_pressed: Option<Box<dyn FnMut(&PushButtonState, MouseButton)>>,
   on_released: Option<Box<dyn FnMut(&PushButtonState, MouseButton)>>,
}

impl PushButtonHandler {
   /// Construct new.
   pub fn new() -> Self {
      Self::default()
   }

   /// Set callback.
   ///
   /// It allocates memory in heap for the closure.
   pub fn on_click(mut self, cb: impl FnMut(&PushButtonState) + 'static) -> Self {
      self.on_click = Some(Box::new(cb));
      self
   }

   /// Set callback.
   ///
   /// It allocates memory in heap for the closure.
   pub fn on_pressed(mut self, cb: impl FnMut(&PushButtonState, MouseButton) + 'static) -> Self {
      self.on_pressed = Some(Box::new(cb));
      self
   }

   /// Set callback.
   ///
   /// It allocates memory in heap for the closure.
   pub fn on_released(mut self, cb: impl FnMut(&PushButtonState, MouseButton) + 'static) -> Self {
      self.on_released = Some(Box::new(cb));
      self
   }
}

impl IPushButtonHandler for PushButtonHandler {
   fn click(&mut self, state: &PushButtonState) {
      if let Some(h) = &mut self.on_click {
         (h)(state)
      }
   }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Default)]
pub struct PushButtonState {
   pub label: Label,
   pub toggle_num: u8,
   pub toggle: u8,
   pub is_hover: bool,
   pub is_down: bool,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct PushButton<H>
where
   H: IPushButtonHandler,
{
   state: PushButtonState,
   handler: H,
}

impl<H> PushButton<H>
where
   H: IPushButtonHandler + 'static,
{
   pub fn new<TXT>(
      handler: H,
      rect: Rect<f32>,
      label: TXT,
      text_patin: TextPaint,
   ) -> Rc<RefCell<Widget<Self>>>
   where
      TXT: Into<Cow<'static, str>>,
   {
      let out = Widget::new(|vt| {
         vt.on_draw = Self::on_draw;
         vt.on_mouse_move = Self::on_mouse_move;
         vt.on_mouse_button = Self::on_mouse_button;

         Self {
            handler,
            state: PushButtonState {
               label: Label::new(
                  label,
                  rect.center(),
                  text_patin,
                  TextAlign::new().center().middle(),
               ),
               toggle_num: 2,
               toggle: 0,
               is_hover: false,
               is_down: false,
            },
         }
      });

      match out.try_borrow_mut() {
         Ok(mut w) => {
            w.set_rect(rect);
         }
         Err(_) => {
            unreachable!()
         }
      }

      out
   }

   #[inline]
   pub fn set_toggle_num(&mut self, num: u8) {
      self.state.toggle_num = num.max(2);
      self.state.toggle = self.state.toggle.min(self.state.toggle_num - 1);
   }

   #[inline]
   pub fn state(&self) -> &PushButtonState {
      &self.state
   }
}

impl<H> Derive for PushButton<H>
where
   H: IPushButtonHandler + 'static,
{
   fn as_any(&self) -> &dyn Any {
      self
   }

   fn as_any_mut(&mut self) -> &mut dyn Any {
      self
   }
}

impl<H> PushButton<H>
where
   H: IPushButtonHandler + 'static,
{
   fn on_draw(w: &mut Widget<PushButton<H>>, canvas: &mut Canvas, _event: &DrawEventCtx) {
      let d = w.derive_ref();

      canvas.set_paint(Paint::new_color(Rgba::GRAY.with_alpha(0.5)));

      if d.state.is_hover {
         canvas.set_color(Rgba::GRAY);
      }

      if d.state.is_down {
         canvas.set_color(Rgba::GRAY_LIGHT);
      }

      canvas.fill(&w.geometry().rect());
      if !d.state.label.text.is_empty() {
         d.state.label.on_draw(canvas);
      }
   }

   pub fn on_mouse_move(w: &mut Widget<PushButton<H>>, event: &MouseMoveEventCtx) -> bool {
      let rect = w.geometry().rect();
      let is_over = rect.is_inside(event.input.x, event.input.y);

      let mut d = w.derive_mut();

      if d.state.is_hover != is_over {
         d.state.is_hover = is_over;
         w.request_draw();
         is_over
      } else {
         false
      }
   }

   pub fn on_mouse_button(w: &mut Widget<PushButton<H>>, event: &MouseButtonsEventCtx) -> bool {
      let mut d = w.derive_mut();

      match event.input.state {
         MouseState::Pressed => {
            if d.state.is_hover {
               d.state.is_down = true;
               d.handler.pressed(&d.state, event.input.button);
               w.request_draw();
               return true;
            }
         }
         MouseState::Released => {
            if d.state.is_down {
               d.state.is_down = false;
               d.handler.released(&d.state, event.input.button);

               if d.state.is_hover {
                  d.state.toggle += 1;
                  if d.state.toggle == d.state.toggle_num {
                     d.state.toggle = 0;
                  }

                  d.handler.click(&d.state);
               }
               w.request_draw();
               return true;
            }
         }
      }

      false
   }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
   use super::*;

   #[test]
   fn sizes() {
      println!(
         "{} : {}",
         std::any::type_name::<PushButton<PushButtonHandler>>(),
         std::mem::size_of::<PushButton<PushButtonHandler>>()
      );
   }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
