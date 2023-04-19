use crate::core::{Brush, Font};
use sim_draw::{color::Rgba, TextAlign};

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Default)]
pub struct Margin {
   pub left: f32,
   pub right: f32,
   pub top: f32,
   pub bottom: f32,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct Text {
   pub font: Font,
   pub align: TextAlign,
   pub margin: Margin,
}

impl Default for Text {
   fn default() -> Self {
      Self { font: Font::default(), align: TextAlign::default(), margin: Margin::default() }
   }
}

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
