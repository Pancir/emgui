//! [developer.mozilla.org]<https://developer.mozilla.org/en-US/docs/Learn/CSS>

use crate::core::{Brush, Font, Painter};
use bitflags::bitflags;
use sim_draw::{
   color::Rgba,
   m::{EdgeInsets, Rect},
   TextAlign,
};

use super::Theme;

////////////////////////////////////////////////////////////////////////////////////////////////////

bitflags! {
   #[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
   pub struct StyleBasicFlags: u32 {
      /// Represents an element (such as a button) that is being activated by the user.
      ///
      /// When using a mouse, "activation" typically starts when
      /// the user presses down the primary mouse button.
      const Active = 1<<0;

      /// Element that is the default in a group of related elements.
      const Default = 1<<1;

      /// Element that has no children.
      const Empty = 1<<2;

      /// Element that has received focus.
      const Focus  = 1<<3;

      /// When the user interacts with an element with a pointing device,
      /// but does not necessarily activate it.
      /// It is generally triggered when the user hovers over an element with
      /// the cursor (mouse pointer).
      const Hover  = 1<<4;

      /// Element whose contents fail to validate.
      const Invalid  = 1<<5;
   }
}

pub trait Style<Data> {
   /// Get the style type name for debugging purposes.
   ///
   /// # Note
   /// Developers should not override this method.
   #[inline]
   fn type_name(&self) -> &'static str {
      std::any::type_name::<Self>()
   }

   /// Get the style type name for debugging purposes.
   ///
   /// # Note
   /// Developers should not override this method.
   #[inline]
   fn type_name_short(&self) -> &'static str {
      let name = self.type_name();
      name.split('<').next().unwrap_or(name).split("::").last().unwrap_or(name)
   }

   //---------------------------------------

   fn name(&self) -> &str;
   fn rect(&self, data: &Data) -> Rect<f32>;
   fn draw_enabled(&self, theme: &Theme, data: &Data, painter: &mut Painter);
   fn draw_disabled(&self, theme: &Theme, data: &Data, painter: &mut Painter);
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct StyleData {
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
