use crate::widgets::events::{MouseButtonsEvent, MouseMoveEvent};
use crate::widgets::{Derive, IWidget, Label, Widget};
use sim_draw::color::Rgba;
use sim_draw::m::Rect;
use sim_draw::{Canvas, TextAlign, TextPaint};
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
   fn pressed(&mut self, _state: &PushButtonState) {}

   /// This is called when the button is released.
   ///
   /// The [PushButtonState::is_down] is `false`.
   fn released(&mut self, _state: &PushButtonState) {}
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
   on_pressed: Option<Box<dyn FnMut(&PushButtonState)>>,
   on_released: Option<Box<dyn FnMut(&PushButtonState)>>,
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
   pub fn on_pressed(mut self, cb: impl FnMut(&PushButtonState) + 'static) -> Self {
      self.on_pressed = Some(Box::new(cb));
      self
   }

   /// Set callback.
   ///
   /// It allocates memory in heap for the closure.
   pub fn on_released(mut self, cb: impl FnMut(&PushButtonState) + 'static) -> Self {
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
   pub is_toggled: bool,
   pub is_hover: bool,
   pub is_down: bool,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct PushButton<HDL>
where
   HDL: IPushButtonHandler,
{
   state: PushButtonState,
   handler: HDL,
}

impl<HDL> PushButton<HDL>
where
   HDL: IPushButtonHandler + 'static,
{
   pub fn new<TXT>(
      handler: HDL,
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
               is_toggled: false,
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
}

impl<HDL> Derive for PushButton<HDL>
where
   HDL: IPushButtonHandler + 'static,
{
   fn as_any(&self) -> &dyn Any {
      self
   }

   fn as_any_mut(&mut self) -> &mut dyn Any {
      self
   }
}

impl<HDL> PushButton<HDL>
where
   HDL: IPushButtonHandler + 'static,
{
   fn on_draw(w: &mut Widget<PushButton<HDL>>, canvas: &mut Canvas) {
      let d = w.derive_ref();

      canvas.set_color(Rgba::GRAY.with_alpha(0.5));

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

   pub fn on_mouse_move(w: &mut Widget<PushButton<HDL>>, event: &MouseMoveEvent) -> bool {
      let rect = w.geometry().rect();
      let mut d = w.derive_mut();
      d.state.is_hover = rect.is_inside(event.input.x, event.input.y);
      d.state.is_hover
   }

   pub fn on_mouse_button(w: &mut Widget<PushButton<HDL>>, event: &MouseButtonsEvent) -> bool {
      let down =
         event.input.state == MouseState::Pressed && event.input.button == MouseButton::Left;

      let mut d = w.derive_mut();

      let is_click = !down && d.state.is_hover && d.state.is_down;
      d.state.is_down = down && d.state.is_hover;
      if is_click {
         d.state.is_toggled = !d.state.is_toggled;
         d.handler.click(&d.state);
      }
      is_click
   }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
