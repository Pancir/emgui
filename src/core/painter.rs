use super::{Brush, Pen};
use sim_draw::Canvas;

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct Painter {
   pen: Pen,
   brush: Brush,
   pub canvas: Box<Canvas>,
}

impl Painter {
   pub fn set_pen(&mut self, pen: Pen) {
      self.pen = pen;
      self.canvas.set_stroke_width(self.pen.width());
   }

   pub fn pen(&self) -> &Pen {
      &self.pen
   }

   pub fn pen_mut(&mut self) -> &mut Pen {
      &mut self.pen
   }

   pub fn set_brush(&mut self, brush: Brush) {
      self.brush = brush;
      self.canvas.set_paint(*self.brush.raw());
   }

   pub fn brush(&self) -> &Brush {
      &self.brush
   }

   pub fn brush_mut(&mut self) -> &mut Brush {
      &mut self.brush
   }
}

// TODO remove when code is ready for it
impl core::ops::Deref for Painter {
   type Target = Canvas;

   fn deref(&self) -> &Self::Target {
      &self.canvas
   }
}

// TODO remove when code is ready for it
impl core::ops::DerefMut for Painter {
   fn deref_mut(&mut self) -> &mut Self::Target {
      &mut self.canvas
   }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
