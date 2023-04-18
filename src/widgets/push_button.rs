use crate::core::events::{DrawEventCtx, MouseButtonsEventCtx};
use crate::core::{IWidget, WidgetRefOwner};
use crate::elements::LineLabel;
use crate::widgets::Widget;
use sim_draw::color::Rgba;
use sim_draw::m::Rect;
use sim_draw::{Canvas, Paint, TextAlign, TextPaint};
use sim_input::mouse::{MouseButton, MouseState};
use std::borrow::Cow;

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
#[allow(clippy::type_complexity)]
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

   fn pressed(&mut self, state: &PushButtonState, mb: MouseButton) {
      if let Some(h) = &mut self.on_pressed {
         (h)(state, mb)
      }
   }

   fn released(&mut self, state: &PushButtonState, mb: MouseButton) {
      if let Some(h) = &mut self.on_released {
         (h)(state, mb)
      }
   }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Default)]
pub struct PushButtonState {
   pub label: LineLabel,
   pub toggle_num: u8,
   pub toggle: u8,
   pub is_hover: bool,
   pub is_down: bool,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

// TODO obsolete.
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
   pub fn new<TXT>(handler: H, rect: Rect<f32>, label: TXT, text_patin: TextPaint) -> WidgetRefOwner
   where
      TXT: Into<Cow<'static, str>>,
   {
      Widget::inherit(
         |vt| {
            vt.on_draw = Self::on_draw;
            vt.on_mouse_cross = Self::on_mouse_cross;
            vt.on_mouse_button = Self::on_mouse_button;

            Self {
               handler,
               state: PushButtonState {
                  label: LineLabel::new()
                     .with_text(label)
                     .with_paint(text_patin)
                     .with_align(TextAlign::new().center().middle().tight())
                     .with_pos(rect.center()),
                  toggle_num: 2,
                  toggle: 0,
                  is_hover: false,
                  is_down: false,
               },
            }
         },
         |w| {
            // TODO check it when draw function is changed.
            // It is enabled for testing..
            // w.set_transparent(true);
            // w.set_mouse_tracking(true);

            w.set_rect(rect);
         },
      )
      .to_owner()
   }

   //---------------------------------------

   #[inline]
   pub fn handler(&self) -> &H {
      &self.handler
   }

   #[inline]
   pub fn handler_mut(&mut self) -> &mut H {
      &mut self.handler
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

impl<H> PushButton<H>
where
   H: IPushButtonHandler + 'static,
{
   fn on_draw(w: &mut Widget<Self>, canvas: &mut Canvas, _event: &DrawEventCtx) {
      let d = w.inherited_obj();

      canvas.set_paint(Paint::new_color(Rgba::GREEN.with_alpha_mul(0.5)));

      if d.state.is_hover {
         canvas.set_color(Rgba::AMBER);
      }

      if d.state.is_down {
         canvas.set_color(Rgba::RED);
      }

      let rect = w.base().geometry().rect();

      canvas.fill(&rect);

      canvas.set_color(Rgba::BLACK);
      canvas.set_aa_fringe(Some(1.0));
      canvas.set_stroke_width(2.0);
      canvas.stroke(&rect);

      if !d.state.label.text().as_ref().is_empty() {
         d.state.label.on_draw(canvas);
      }
   }

   pub fn on_mouse_cross(w: &mut Widget<Self>, enter: bool) {
      w.base().request_draw();
      w.inherited_obj_mut().state.is_hover = enter;
   }

   pub fn on_mouse_button(w: &mut Widget<Self>, event: &MouseButtonsEventCtx) -> bool {
      let mut d = w.inherited_obj_mut();

      match event.input.state {
         MouseState::Pressed => {
            if d.state.is_hover {
               d.state.is_down = true;
               d.handler.pressed(&d.state, event.input.button);
               w.base().request_draw();
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
               w.base().request_draw();
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
      dbg!(std::mem::size_of::<PushButton<PushButtonHandler>>());
      dbg!(std::mem::size_of::<Widget<PushButton<PushButtonHandler>>>());
   }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
