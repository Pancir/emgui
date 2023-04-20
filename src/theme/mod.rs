//! [css_examples](https://www.w3schools.com/css/css_examples.asp)

pub mod style;

mod text_align;
mod theme;
mod theme_colors;
mod theme_render_objects;
mod theme_extensions;
mod theme_fonts;

pub use text_align::*;
pub use theme::*;
pub use theme_colors::*;
pub use theme_render_objects::*;
pub use theme_extensions::*;
pub use theme_fonts::*;

// use sim_draw::color::Rgba8;
// use sim_draw::{Canvas, FontStyle, TextPaint};

////////////////////////////////////////////////////////////////////////////////////////////////////

// pub struct TextStyle<'str> {
//    pub font_family: &'str str,
//    pub color: Rgba8,
//    pub letter_spacing: f32,
//    pub line_height: f32,
// }

// impl<'str> Default for TextStyle<'str> {
//    fn default() -> Self {
//       Self::new()
//    }
// }

// impl<'str> TextStyle<'str> {
//    pub const fn new() -> Self {
//       Self { font_family: "Roboto", color: Rgba8::WHITE, letter_spacing: 1.0, line_height: 1.0 }
//    }
// }

////////////////////////////////////////////////////////////////////////////////////////////////////

// pub fn default_text(canvas: &mut Canvas, fs: &FontStyle, tp: &TextPaint) {}

////////////////////////////////////////////////////////////////////////////////////////////////////

// pub struct RectAreaDraw {
//    pub is_hover: bool,
//    pub is_down: bool,
// }

// pub fn default_area(canvas: &mut Canvas, data: RectAreaDraw) {}

////////////////////////////////////////////////////////////////////////////////////////////////////
