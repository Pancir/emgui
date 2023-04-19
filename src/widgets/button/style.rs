use crate::{
   core::{Brush, Painter, Pen},
   elements::Icon,
   theme::{
      style::{self, Style, StyleBase},
      Theme,
   },
};
use sim_draw::{color::Rgba, m::Rect};
use std::any::Any;

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct ButtonStyleData<'refs> {
   pub text: Option<&'refs str>,
   pub icon: Option<&'refs Icon>,
   pub bounds: Rect<f32>,
   pub is_hover: bool,
   pub is_active: bool,
   pub has_focus: bool,
   pub toggle_num: u8,
   pub toggle_curr: u8,
}

pub trait ButtonStyleSheet: for<'refs> Style<ButtonStyleData<'refs>> {}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Default)]
pub struct ButtonStyle {
   pub text: style::Text,
}

impl ButtonStyle {
   pub fn new_normal() -> Self {
      ButtonStyle::default()
   }

   pub fn new_accent() -> Self {
      ButtonStyle::default()
   }
}

impl StyleBase for ButtonStyle {
   fn as_any(&self) -> &dyn Any {
      self
   }

   fn as_any_mut(&mut self) -> &mut dyn Any {
      self
   }

   fn name(&self) -> &str {
      "button"
   }
}

impl Style<ButtonStyleData<'_>> for ButtonStyle {
   fn rect(&self, data: &ButtonStyleData) -> Rect<f32> {
      data.bounds
   }

   fn draw_disabled(&self, theme: &Theme, data: &ButtonStyleData, painter: &mut Painter) {
      self.draw_enabled(theme, data, painter);
   }

   fn draw_enabled(&self, _theme: &Theme, data: &ButtonStyleData, painter: &mut Painter) {
      painter.set_brush(Brush::new_color(Rgba::GREEN.with_alpha_mul(0.5)));

      if data.is_hover {
         painter.set_brush(Brush::new_color(Rgba::AMBER));
      }

      if data.is_active {
         painter.set_brush(Brush::new_color(Rgba::RED));
      }

      painter.fill(&data.bounds);

      painter.set_pen(Pen::new().with_width(2.0).with_color(Rgba::BLACK));
      painter.stroke(&data.bounds);

      // FIXME needs a style system to fix.
      //   if !w.state.label.text().as_ref().is_empty() {
      //      w.state.label.on_draw(canvas);
      //   }
   }
}

impl ButtonStyleSheet for ButtonStyle {}

////////////////////////////////////////////////////////////////////////////////////////////////////
