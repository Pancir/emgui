use crate::widgets::Label;
use sim_draw::color::Rgba;
use sim_draw::m::Rect;
use sim_draw::{Canvas, TextAlign, TextPaint};
use sim_input::mouse::{MouseButton, MouseState};
use sim_run::{MouseButtonsEvent, MouseMoveEvent};
use std::borrow::Cow;

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct PushButton {
   label: Label,
   rect: Rect<f32>,
   is_toggle: bool,
   is_hover: bool,
   is_down: bool,
   v_on_draw: fn(&mut Self, &mut Canvas),
}

impl Default for PushButton {
   fn default() -> Self {
      Self::new(Rect::new(0.0, 0.0, 100.0, 100.0), "", TextPaint::default())
   }
}

impl PushButton {
   pub fn new<TXT>(rect: Rect<f32>, label: TXT, text_patin: TextPaint) -> Self
   where
      TXT: Into<Cow<'static, str>>,
   {
      let align = TextAlign::new().center().middle();
      let label = Label::new(label, rect.center(), text_patin, align);

      Self { label, rect, is_toggle: false, is_hover: false, is_down: false, v_on_draw: Self::draw }
   }

   pub fn set_text_patin(&mut self, paint: TextPaint) {
      self.label.paint = paint;
   }

   pub fn label(&self) -> &Label {
      &self.label
   }

   pub fn rect(&self) -> Rect<f32> {
      self.rect
   }

   pub fn set_text<TXT>(&mut self, text: TXT)
   where
      TXT: Into<Cow<'static, str>>,
   {
      self.label.text = text.into();
   }

   pub fn text(&self) -> &str {
      self.label.text.as_ref()
   }

   pub fn set_toggle(&mut self, state: bool) {
      self.is_toggle = state;
   }

   pub fn is_down(&self) -> bool {
      self.is_down
   }

   pub fn is_hover(&self) -> bool {
      self.is_hover
   }
}

impl PushButton {
   fn draw(w: &mut Self, canvas: &mut Canvas) {
      canvas.set_color(Rgba::GRAY.with_alpha(0.5));

      if w.is_hover() {
         canvas.set_color(Rgba::GRAY);
      }

      if w.is_down() {
         canvas.set_color(Rgba::GRAY_LIGHT);
      }

      canvas.fill(&w.rect);
      if !w.label().text.is_empty() {
         w.label().on_draw(canvas);
      }
   }
}

impl PushButton {
   #[inline]
   pub fn on_draw(&mut self, canvas: &mut Canvas) {
      (self.v_on_draw)(self, canvas);
   }

   /// Return `true` if mouse is over.
   #[inline]
   #[must_use]
   pub fn on_mouse_move(&mut self, event: &MouseMoveEvent) -> bool {
      self.is_hover = self.rect.is_inside(event.input.x, event.input.y);
      self.is_hover
   }

   /// Return `true` if click is detected.
   #[inline]
   #[must_use]
   pub fn on_mouse_button(&mut self, event: &MouseButtonsEvent) -> bool {
      let down =
         event.input.state == MouseState::Pressed && event.input.button == MouseButton::Left;

      let is_click = !down && self.is_hover && self.is_down;
      self.is_down = down && self.is_hover;
      if is_click {
         self.is_toggle = !self.is_toggle;
      }
      is_click
   }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
