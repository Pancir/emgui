use sim_draw::m::{Point2, Rect};

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct PushButton {
   pub rect: Rect<f32>,
   pub is_hover: bool,
   pub is_down: bool,
}

impl Default for PushButton {
   fn default() -> Self {
      Self::new(Rect::new(0.0, 0.0, 100.0, 100.0))
   }
}

impl PushButton {
   #[inline]
   pub const fn new(rect: Rect<f32>) -> Self {
      Self { rect, is_hover: false, is_down: false }
   }
}

impl PushButton {
   /// Return `true` if mouse is over.
   #[inline]
   pub fn on_mouse_move(&mut self, pos: Point2<f32>) -> bool {
      self.is_hover = self.rect.is_inside(pos.x, pos.y);
      self.is_hover
   }

   /// Return `true` if click is detected.
   #[inline]
   pub fn on_mouse_button(&mut self, down: bool) -> bool {
      let is_click = !down && self.is_hover && self.is_down;
      self.is_down = down && self.is_hover;
      is_click
   }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
