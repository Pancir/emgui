use crate::{
   core::{derive::Derive, events::DrawEventCtx, IWidget},
   widgets::{Button, IButtonHandler},
};
use sim_draw::{color::Rgba, Canvas, Paint, TextAlign, TextPaint};

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct ButtonStyle {
   paint: TextPaint,
   align: TextAlign,
}

impl ButtonStyle {
   fn on_draw<H, D>(&self, w: &Button<H, D>, canvas: &mut Canvas, _event: &DrawEventCtx)
   where
      H: IButtonHandler + 'static,
      D: Derive,
   {
      canvas.set_paint(Paint::new_color(Rgba::GREEN.with_alpha_mul(0.5)));

      if w.state().is_hover {
         canvas.set_color(Rgba::AMBER);
      }

      if w.state().is_down {
         canvas.set_color(Rgba::RED);
      }

      let rect = w.base().geometry().rect();

      canvas.fill(&rect);

      canvas.set_color(Rgba::BLACK);
      canvas.set_aa_fringe(Some(1.0));
      canvas.set_stroke_width(2.0);
      canvas.stroke(&rect);

      let state = w.state();

      if let Some(text) = &state.text {
         canvas.set_text_paint(self.paint.clone());
         canvas.text_line(text.as_ref(), self.pos, self.align);
      }
   }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
