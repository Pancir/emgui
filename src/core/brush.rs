use sim_draw::{color::Rgba, Paint};

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Copy)]
pub struct Brush {
   p: Paint,
}

impl Default for Brush {
   fn default() -> Self {
      Self::new()
   }
}

impl Brush {
   #[inline]
   pub const fn new() -> Self {
      Self::new_color(Rgba::RED)
   }

   #[inline]
   pub const fn new_color(color: Rgba) -> Self {
      Self { p: Paint::new_color(color) }
   }

   #[inline]
   pub fn set_color(&mut self, color: Rgba) {
      *self = Self::new_color(color);
   }

   #[inline]
   // TODO remove when code is ready for it
   pub const fn raw(&self) -> &Paint {
      &self.p
   }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
   use super::*;

   #[test]
   fn sizes() {
      dbg!(std::mem::size_of::<Brush>());
   }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
