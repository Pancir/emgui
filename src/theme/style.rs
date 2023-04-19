//! [developer.mozilla.org]<https://developer.mozilla.org/en-US/docs/Learn/CSS>

use crate::core::{Brush, Font};
use sim_draw::{color::Rgba, m::EdgeInsets, TextAlign};

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct Style {
   pub background: Option<Background>,
   pub border: Option<Border>,
   pub edge_insets: EdgeInsets<f32>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct Text {
   pub font: Font,
   pub color: Rgba,
   pub align: TextAlign,
   pub edge_insets: EdgeInsets<f32>,
}

impl Default for Text {
   fn default() -> Self {
      Self {
         font: Font::default(),
         color: Rgba::RED,
         align: TextAlign::default(),
         edge_insets: EdgeInsets::ZERO,
      }
   }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct Border {}

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

#[cfg(test)]
mod tests {
   use super::*;

   #[test]
   fn sizes() {
      dbg!(std::mem::size_of::<Text>());
      dbg!(std::mem::size_of::<Background>());
   }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
