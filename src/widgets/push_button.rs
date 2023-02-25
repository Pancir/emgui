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

pub struct PushButton {
   label: Label,
   is_toggle: bool,
   is_hover: bool,
   is_down: bool,
}

impl PushButton {
   pub fn new<TXT>(rect: Rect<f32>, label: TXT, text_patin: TextPaint) -> Rc<RefCell<Widget<Self>>>
   where
      TXT: Into<Cow<'static, str>>,
   {
      let out = Widget::new(|vt| {
         vt.on_draw = Self::on_draw;
         vt.on_mouse_move = Self::on_mouse_move;
         vt.on_mouse_button = Self::on_mouse_button;

         Self {
            label: Label::new(label, rect.center(), text_patin, TextAlign::new().center().middle()),
            is_toggle: false,
            is_hover: false,
            is_down: false,
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

impl Derive for PushButton {
   fn as_any(&self) -> &dyn Any {
      self
   }

   fn as_any_mut(&mut self) -> &mut dyn Any {
      self
   }
}

impl PushButton {
   fn on_draw(w: &mut Widget<PushButton>, canvas: &mut Canvas) {
      let d = w.derive_ref();

      canvas.set_color(Rgba::GRAY.with_alpha(0.5));

      if d.is_hover {
         canvas.set_color(Rgba::GRAY);
      }

      if d.is_down {
         canvas.set_color(Rgba::GRAY_LIGHT);
      }

      canvas.fill(&w.geometry().rect());
      if !d.label.text.is_empty() {
         d.label.on_draw(canvas);
      }
   }

   pub fn on_mouse_move(w: &mut Widget<PushButton>, event: &MouseMoveEvent) -> bool {
      let rect = w.geometry().rect();
      let mut d = w.derive_mut();
      d.is_hover = rect.is_inside(event.input.x, event.input.y);
      d.is_hover
   }

   pub fn on_mouse_button(w: &mut Widget<PushButton>, event: &MouseButtonsEvent) -> bool {
      let down =
         event.input.state == MouseState::Pressed && event.input.button == MouseButton::Left;

      let mut d = w.derive_mut();

      let is_click = !down && d.is_hover && d.is_down;
      d.is_down = down && d.is_hover;
      if is_click {
         d.is_toggle = !d.is_toggle;
      }
      is_click
   }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
