use crate::core::Brush;
use sim_draw::color::Rgba;

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct Background {
   brush: Brush,
}

impl Background {
   pub const fn new_color(color: Rgba) -> Self {
      Self { brush: Brush::new_color(color) }
   }

   pub const fn new(brush: Brush) -> Self {
      Self { brush }
   }

   pub const fn brush(&self) -> &Brush {
      &self.brush
   }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
