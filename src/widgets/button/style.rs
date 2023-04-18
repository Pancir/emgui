use super::ButtonState;
use crate::core::events::DrawEventCtx;
use sim_draw::{m::Rect, Canvas};

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct ButtonStyleState<'internal> {
   pub rect: Rect<f32>,
   pub state: &'internal ButtonState,
}

pub trait ButtonStyleSheet {
   fn rect(&self, state: &ButtonStyleState) -> Rect<f32>;
   fn draw(&self, state: &ButtonStyleState, canvas: &mut Canvas, _event: &DrawEventCtx);
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct ButtonStyle {}

impl ButtonStyleSheet for ButtonStyle {
   fn rect(&self, state: &ButtonStyleState) -> Rect<f32> {
      state.rect
   }

   fn draw(&self, state: &ButtonStyleState, canvas: &mut Canvas, _event: &DrawEventCtx) {
      // canvas.set_paint(Paint::new_color(Rgba::GREEN.with_alpha_mul(0.5)));

      // if w.state.is_hover {
      //    canvas.set_color(Rgba::AMBER);
      // }

      // if w.state.is_down {
      //    canvas.set_color(Rgba::RED);
      // }

      // let rect = w.base.geometry().rect();

      // canvas.fill(&rect);

      // canvas.set_color(Rgba::BLACK);
      // canvas.set_aa_fringe(Some(1.0));
      // canvas.set_stroke_width(2.0);
      // canvas.stroke(&rect);

      // FIXME needs a style system to fix.
      //   if !w.state.label.text().as_ref().is_empty() {
      //      w.state.label.on_draw(canvas);
      //   }
   }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
