use sim_draw::m::{Box2, Point2};
use sim_draw::{Canvas, TextAlign, TextPaint};
use std::borrow::Cow;

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct Label {
   pub pos: Point2<f32>,
   pub text: Cow<'static, str>,
   pub paint: TextPaint,
   pub align: TextAlign,
}

impl Default for Label {
   fn default() -> Self {
      Self {
         pos: Point2::new(0.0, 0.0),
         text: Cow::Borrowed(""),
         paint: Default::default(),
         align: Default::default(),
      }
   }
}

impl Label {
   pub fn new<TXT>(text: TXT, pos: Point2<f32>, paint: TextPaint, align: TextAlign) -> Self
   where
      TXT: Into<Cow<'static, str>>,
   {
      Self { pos, text: text.into(), paint, align }
   }

   pub fn on_draw(&self, canvas: &mut Canvas) -> Box2<f32> {
      canvas.set_text_paint(self.paint.clone());
      canvas.text_line(self.text.as_ref(), self.pos, self.align)
   }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
