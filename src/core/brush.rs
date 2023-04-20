use super::Rgba;

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Copy)]
pub struct Brush {}

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
      unimplemented!()
   }

   #[inline]
   pub fn set_color(&mut self, color: Rgba) {
      *self = Self::new_color(color);
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
