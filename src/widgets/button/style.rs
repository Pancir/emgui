use super::{ButtonState, ButtonStateFlags};
use crate::{
   core::{Brush, Painter, Pen, WidgetBase},
   theme::{
      style::{self, Style, StyleBase},
      Theme,
   },
};
use sim_draw::{color::Rgba, m::Rect};
use std::any::Any;

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct ButtonStyleData<'internal> {
   pub base: &'internal WidgetBase,
   pub state: &'internal ButtonState,
}

impl<'internal> ButtonStyleData<'internal> {
   #[inline]
   pub fn is_over(&self) -> bool {
      self.base.is_over()
   }

   #[inline]
   pub fn is_down(&self) -> bool {
      self.state.flags.contains(ButtonStateFlags::IS_DOWN)
   }

   #[inline]
   pub fn is_enabled(&self) -> bool {
      unimplemented!()
   }

   #[inline]
   pub fn has_focus(&self) -> bool {
      unimplemented!()
   }

   #[inline]
   pub fn rect(&self) -> Rect<f32> {
      self.base.geometry().rect()
   }
}

pub trait ButtonStyleSheet: for<'data> Style<ButtonStyleData<'data>> {}

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
      data.rect()
   }

   fn draw_disabled(&self, theme: &Theme, data: &ButtonStyleData, painter: &mut Painter) {
      self.draw_enabled(theme, data, painter);
   }

   fn draw_enabled(&self, _theme: &Theme, data: &ButtonStyleData, painter: &mut Painter) {
      let rect = data.rect();
      painter.set_brush(Brush::new_color(Rgba::GREEN.with_alpha_mul(0.5)));

      if data.is_over() {
         painter.set_brush(Brush::new_color(Rgba::AMBER));
      }

      if data.is_down() {
         painter.set_brush(Brush::new_color(Rgba::RED));
      }

      painter.fill(&rect);

      painter.set_pen(Pen::new().with_width(2.0).with_color(Rgba::BLACK));
      painter.stroke(&rect);

      // FIXME needs a style system to fix.
      //   if !w.state.label.text().as_ref().is_empty() {
      //      w.state.label.on_draw(canvas);
      //   }
   }
}

impl ButtonStyleSheet for ButtonStyle {}

////////////////////////////////////////////////////////////////////////////////////////////////////
