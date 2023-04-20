use super::Rgba;

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Copy)]
pub struct Brush {
   color: Rgba,
}

impl Default for Brush {
   fn default() -> Self {
      Self::new()
   }
}

impl Brush {
   #[inline]
   pub fn new() -> Self {
      Self::new_color(Rgba::RED)
   }

   #[inline]
   pub fn new_color<Color>(color: Color) -> Self
   where
      Color: Into<Rgba>,
   {
      Self { color: color.into() }
   }

   #[inline]
   pub fn set_color<Color>(&mut self, color: Color)
   where
      Color: Into<Rgba>,
   {
      self.color = color.into()
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
