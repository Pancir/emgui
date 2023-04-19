use super::{ButtonState, ButtonStateFlags};
use crate::{
   core::{Brush, Painter, Pen, WidgetBase},
   theme::style,
};
use sim_draw::{color::Rgba, m::Rect};

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct ButtonStyleState<'internal> {
   pub base: &'internal WidgetBase,
   pub state: &'internal ButtonState,
   pub painter: &'internal mut Painter,
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

pub trait ButtonStyleSheet {
   fn rect(&self, state: &ButtonStyleState) -> Rect<f32> {
      state.rect()
   }

   fn draw(&self, state: &mut ButtonStyleState);
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Default)]
pub struct ButtonStyle {
   pub text: style::Text,
}

impl ButtonStyleSheet for ButtonStyle {
   fn draw(&self, state: &mut ButtonStyleState) {
      let rect = state.rect();
      state.painter.set_brush(Brush::new_color(Rgba::GREEN.with_alpha_mul(0.5)));

      if state.is_over() {
         state.painter.set_brush(Brush::new_color(Rgba::AMBER));
      }

      if state.is_down() {
         state.painter.set_brush(Brush::new_color(Rgba::RED));
      }

      state.painter.fill(&rect);

      state.painter.set_pen(Pen::new().with_width(2.0).with_color(Rgba::BLACK));
      state.painter.stroke(&rect);

      // FIXME needs a style system to fix.
      //   if !w.state.label.text().as_ref().is_empty() {
      //      w.state.label.on_draw(canvas);
      //   }
   }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
