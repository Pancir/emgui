use super::{ButtonState, ButtonStateFlags};
use crate::{
   core::{Brush, Painter, Pen, WidgetBase},
   theme::{
      style::{self, Style},
      Theme,
   },
};
use sim_draw::{color::Rgba, m::Rect};

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct ButtonStyleState<'internal> {
   pub base: &'internal WidgetBase,
   pub state: &'internal ButtonState,
}

impl<'internal> ButtonStyleState<'internal> {
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

pub trait ButtonStyleSheet: for<'data> Style<ButtonStyleState<'data>> {}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Default)]
pub struct ButtonStyle {
   pub text: style::Text,
}

impl Style<ButtonStyleState<'_>> for ButtonStyle {
   fn name(&self) -> &str {
      "button_default"
   }

   fn rect(&self, data: &ButtonStyleState) -> Rect<f32> {
      data.rect()
   }

   fn draw_disabled(&self, theme: &Theme, data: &ButtonStyleState, painter: &mut Painter) {
      self.draw_enabled(theme, data, painter);
   }

   fn draw_enabled(&self, _theme: &Theme, data: &ButtonStyleState, painter: &mut Painter) {
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
