use super::{ButtonState, ButtonStateFlags};
use crate::core::{Painter, WidgetBase};
use sim_draw::{color::Rgba, m::Rect, Paint};

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct ButtonStyleState<'internal> {
   pub base: &'internal WidgetBase,
   pub state: &'internal ButtonState,
   pub canvas: &'internal mut Painter,
}

pub trait ButtonStyleSheet {
   fn rect(&self, state: &ButtonStyleState) -> Rect<f32> {
      state.base.geometry().rect()
   }

   fn draw(&self, state: &mut ButtonStyleState);
}

////////////////////////////////////////////////////////////////////////////////////////////////////
///
#[derive(Default)]
pub struct ButtonStyle {}

impl ButtonStyleSheet for ButtonStyle {
   fn draw(&self, state: &mut ButtonStyleState) {
      state.canvas.set_paint(Paint::new_color(Rgba::GREEN.with_alpha_mul(0.5)));

      if state.base.is_over() {
         state.canvas.set_color(Rgba::AMBER);
      }

      if state.state.flags.contains(ButtonStateFlags::IS_DOWN) {
         state.canvas.set_color(Rgba::RED);
      }

      let rect = state.base.geometry().rect();

      state.canvas.fill(&rect);

      state.canvas.set_color(Rgba::BLACK);
      state.canvas.set_aa_fringe(Some(1.0));
      state.canvas.set_stroke_width(2.0);
      state.canvas.stroke(&rect);

      // FIXME needs a style system to fix.
      //   if !w.state.label.text().as_ref().is_empty() {
      //      w.state.label.on_draw(canvas);
      //   }
   }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
