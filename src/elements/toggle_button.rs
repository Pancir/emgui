use sim_draw::m::{Box2, Point2};

////////////////////////////////////////////////////////////////////////////////////////////////////

/// Push button.
///
/// It detects toggle when mouse is released over button.
#[derive(Copy, Clone, Debug)]
pub struct ToggleButton {
   pub rect: Box2<f32>,
   pub is_hover: bool,
   pub is_down: bool,
   pub is_toggle: bool,
}

impl Default for ToggleButton {
   fn default() -> Self {
      Self::new(Box2::new_xy_xy(0.0, 0.0, 100.0, 100.0))
   }
}

impl ToggleButton {
   /// Construct new.
   #[inline]
   pub const fn new(rect: Box2<f32>) -> Self {
      Self { rect, is_hover: false, is_toggle: false, is_down: false }
   }
}

impl ToggleButton {
   /// Return `true` if mouse is over.
   #[inline]
   pub fn on_mouse_move(&mut self, pos: Point2<f32>) -> bool {
      self.is_hover = self.rect.is_inside(pos.x, pos.y);
      self.is_hover
   }

   /// Return `true` if toggled (not a toggle state!).
   #[inline]
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

#[cfg(test)]
mod tests {
   use super::*;
   use sim_draw::m::Rect;

   #[test]
   fn toggle_button() {
      {
         let mut b = ToggleButton::new(Rect::new(50.0, 50.0, 100.0, 100.0).into());
         assert!(!b.on_mouse_move(Point2::new(0.0, 0.0)));
         assert!(!b.on_mouse_button(true));
         assert!(!b.on_mouse_button(false));
         assert!(!b.is_toggle);
      }

      {
         let mut b = ToggleButton::new(Rect::new(50.0, 50.0, 100.0, 100.0).into());
         assert!(!b.on_mouse_move(Point2::new(0.0, 0.0)));
         assert!(!b.on_mouse_button(true));
         assert!(b.on_mouse_move(Point2::new(75.0, 75.0)));
         assert!(!b.on_mouse_button(false));
         assert!(!b.is_toggle);
      }

      {
         let mut b = ToggleButton::new(Rect::new(50.0, 50.0, 100.0, 100.0).into());
         assert!(b.on_mouse_move(Point2::new(75.0, 75.0)));
         assert!(!b.on_mouse_button(true));
         assert!(!b.on_mouse_move(Point2::new(0.0, 0.0)));
         assert!(!b.on_mouse_button(false));
         assert!(!b.is_toggle);
      }

      {
         let mut b = ToggleButton::new(Rect::new(50.0, 50.0, 100.0, 100.0).into());
         assert!(b.on_mouse_move(Point2::new(75.0, 75.0)));
         assert!(!b.on_mouse_button(true));
         assert!(b.on_mouse_button(false));
         assert!(b.is_toggle);

         assert!(b.on_mouse_move(Point2::new(75.0, 75.0)));
         assert!(!b.on_mouse_button(true));
         assert!(b.on_mouse_button(false));
         assert!(!b.is_toggle);
      }
   }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
