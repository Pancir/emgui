use sim_draw::m::{Box2, Point2};

////////////////////////////////////////////////////////////////////////////////////////////////////

/// Push button.
///
/// It detects click when mouse is released over button.
#[derive(Copy, Clone, Debug)]
pub struct PushButton {
   pub rect: Box2<f32>,
   pub is_hover: bool,
   pub is_down: bool,
}

impl Default for PushButton {
   fn default() -> Self {
      Self::new(Box2::new_xy_xy(0.0, 0.0, 100.0, 100.0))
   }
}

impl PushButton {
   /// Construct new.
   #[inline]
   pub const fn new(rect: Box2<f32>) -> Self {
      Self { rect, is_hover: false, is_down: false }
   }
}

impl PushButton {
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
      is_click
   }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
   use super::*;
   use sim_draw::m::Rect;

   #[test]
   fn push_button() {
      {
         let mut b = PushButton::new(Rect::new(50.0, 50.0, 100.0, 100.0).into());
         assert!(!b.on_mouse_move(Point2::new(0.0, 0.0)));
         assert!(!b.on_mouse_button(true));
         assert!(!b.on_mouse_button(false));
      }

      {
         let mut b = PushButton::new(Rect::new(50.0, 50.0, 100.0, 100.0).into());
         assert!(!b.on_mouse_move(Point2::new(0.0, 0.0)));
         assert!(!b.on_mouse_button(true));
         assert!(b.on_mouse_move(Point2::new(75.0, 75.0)));
         assert!(!b.on_mouse_button(false));
      }

      {
         let mut b = PushButton::new(Rect::new(50.0, 50.0, 100.0, 100.0).into());
         assert!(b.on_mouse_move(Point2::new(75.0, 75.0)));
         assert!(!b.on_mouse_button(true));
         assert!(!b.on_mouse_move(Point2::new(0.0, 0.0)));
         assert!(!b.on_mouse_button(false));
      }

      {
         let mut b = PushButton::new(Rect::new(50.0, 50.0, 100.0, 100.0).into());
         assert!(b.on_mouse_move(Point2::new(75.0, 75.0)));
         assert!(!b.on_mouse_button(true));
         assert!(b.on_mouse_button(false));
      }
   }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
