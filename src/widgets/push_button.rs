use crate::widgets::Label;
use sim_draw::color::Rgba;
use sim_draw::m::{Point2, Rect};
use sim_draw::Canvas;
use std::borrow::Cow;

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct PushButton {
   label: Option<Label>,
   rect: Rect<f32>,
   is_toggle: bool,
   is_hover: bool,
   is_down: bool,
   v_on_draw: fn(&mut Self, &mut Canvas),
}

impl Default for PushButton {
   fn default() -> Self {
      Self::new(Rect::new(0.0, 0.0, 100.0, 100.0), None)
   }
}

impl PushButton {
   pub fn new(rect: Rect<f32>, label: Option<Label>) -> Self {
      Self { label, rect, is_toggle: false, is_hover: false, is_down: false, v_on_draw: Self::draw }
   }

   pub fn label(&self) -> &Option<Label> {
      &self.label
   }

   pub fn rect(&self) -> Rect<f32> {
      self.rect
   }

   pub fn set_text<TXT>(&mut self, text: TXT)
   where
      TXT: Into<Cow<'static, str>>,
   {
      if let Some(l) = self.label.as_mut() {
         l.text = text.into()
      }
   }

   pub fn text(&self) -> &str {
      if let Some(l) = self.label.as_ref() {
         l.text.as_ref()
      } else {
         ""
      }
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
      if let Some(l) = w.label() {
         l.draw(canvas);
      }
   }
}

impl PushButton {
   #[inline]
   pub fn on_draw(w: &mut Self, canvas: &mut Canvas) {
      (w.v_on_draw)(w, canvas);
   }

   /// Return `true` if mouse is over.
   #[inline]
   #[must_use]
   pub fn on_mouse_move(&mut self, pos: Point2<f32>) -> bool {
      self.is_hover = self.rect.is_inside(pos.x, pos.y);
      self.is_hover
   }

   /// Return `true` if click is detected.
   #[inline]
   #[must_use]
   pub fn on_mouse_button(&mut self, down: bool) -> bool {
      let is_click = !down && self.is_hover && self.is_down;
      self.is_down = down && self.is_hover;
      if is_click {
         self.is_toggle = !self.is_toggle;
      }
      is_click
   }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
