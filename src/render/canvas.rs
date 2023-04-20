use super::{Brush, Pen};
use crate::theme::TextAlign;
use m::{Box2, Point2};

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct Canvas {
   pen: Pen,
   brush: Brush,
}

impl Canvas {
   pub fn set_pen(&mut self, pen: Pen) {
      self.pen = pen;
   }

   pub fn pen(&self) -> &Pen {
      &self.pen
   }

   pub fn pen_mut(&mut self) -> &mut Pen {
      &mut self.pen
   }

   pub fn set_brush(&mut self, brush: Brush) {
      self.brush = brush;
   }

   pub fn brush(&self) -> &Brush {
      &self.brush
   }

   pub fn brush_mut(&mut self) -> &mut Brush {
      &mut self.brush
   }
}

impl Canvas {
   pub fn fill_text_line(&mut self, text: &str, pos: Point2<f32>, align: TextAlign) -> Box2<f32> {
      unimplemented!()
   }

   pub fn fill_text_block(&mut self, text: &str, pos: Point2<f32>, align: TextAlign) -> Box2<f32> {
      unimplemented!()
   }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
