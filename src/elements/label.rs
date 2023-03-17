use sim_draw::m::{Box2, Point2};
use sim_draw::{Canvas, TextAlign, TextPaint};
use std::borrow::Cow;

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct LineLabel {
   pos: Point2<f32>,
   text: Cow<'static, str>,
   paint: TextPaint,
   align: TextAlign,
}

impl Default for LineLabel {
   fn default() -> Self {
      Self::new()
   }
}

impl LineLabel {
   #[must_use]
   #[inline]
   pub fn new() -> Self {
      Self {
         pos: Point2::ZERO,
         text: Cow::Borrowed(""),
         paint: Default::default(),
         align: Default::default(),
      }
   }

   #[must_use]
   #[inline]
   pub fn with_text<TXT>(mut self, text: TXT) -> Self
   where
      TXT: Into<Cow<'static, str>>,
   {
      self.set_text(text);
      self
   }

   #[must_use]
   #[inline]
   pub fn with_pos(mut self, pos: Point2<f32>) -> Self {
      self.set_pos(pos);
      self
   }

   #[must_use]
   #[inline]
   pub fn with_paint(mut self, paint: TextPaint) -> Self {
      self.set_paint(paint);
      self
   }

   #[must_use]
   #[inline]
   pub fn with_align(mut self, align: TextAlign) -> Self {
      self.set_align(align);
      self
   }
}

impl LineLabel {
   #[inline]
   pub fn set_text<TXT>(&mut self, text: TXT)
   where
      TXT: Into<Cow<'static, str>>,
   {
      self.text = text.into()
   }

   #[inline]
   pub fn text(&self) -> &Cow<'static, str> {
      &self.text
   }

   #[inline]
   pub fn set_pos(&mut self, pos: Point2<f32>) {
      self.pos = pos;
   }

   #[inline]
   pub fn pos(&self) -> Point2<f32> {
      self.pos
   }

   #[inline]
   pub fn set_paint(&mut self, paint: TextPaint) {
      self.paint = paint;
   }

   #[inline]
   pub fn paint(&self) -> &TextPaint {
      &self.paint
   }

   #[inline]
   pub fn set_align(&mut self, align: TextAlign) {
      self.align = align;
   }

   #[inline]
   pub fn align(&self) -> TextAlign {
      self.align
   }
}

impl LineLabel {
   #[inline]
   pub fn on_draw(&self, canvas: &mut Canvas) -> Box2<f32> {
      canvas.set_text_paint(self.paint.clone());
      canvas.text_line(self.text.as_ref(), self.pos, self.align)
   }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
