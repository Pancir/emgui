use super::{Brush, Rgba};

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Copy)]
pub struct Pen {
   brush: Brush,
   width: f32,
}

impl Default for Pen {
   fn default() -> Self {
      Pen::new()
   }
}

impl Pen {
   #[inline]
   pub fn new() -> Self {
      Self { width: 2.0, brush: Brush::new() }
   }

   #[inline]
   pub const fn with_width(mut self, width: f32) -> Self {
      self.width = width;
      self
   }

   #[inline]
   pub fn set_width(&mut self, width: f32) {
      self.width = width;
   }

   #[inline]
   pub const fn width(&self) -> f32 {
      self.width
   }

   #[inline]
   pub fn with_color<Color>(mut self, color: Color) -> Self
   where
      Color: Into<Rgba>,
   {
      self.brush = Brush::new_color(color);
      self
   }

   #[inline]
   pub fn set_color(&mut self, color: Rgba) {
      self.brush = Brush::new_color(color);
   }

   #[inline]
   pub const fn with_brush(mut self, brush: Brush) -> Self {
      self.brush = brush;
      self
   }

   #[inline]
   pub fn set_brush(&mut self, brush: Brush) {
      self.brush = brush;
   }

   #[inline]
   pub const fn brush(&self) -> &Brush {
      &self.brush
   }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
   use super::*;

   #[test]
   fn sizes() {
      dbg!(std::mem::size_of::<Pen>());
   }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
