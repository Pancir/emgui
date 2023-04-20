use super::{Brush, Pen};
use crate::{backend, theme::TextAlign};
use m::{Box2, Point2};
use std::{any::Any, rc::Rc};

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct Canvas {
   user_data: Option<Rc<dyn Any>>,
   inner: Rc<dyn backend::Canvas>,
   pen: Pen,
   brush: Brush,
}

impl Canvas {
   pub fn new<C, U>(canvas: Rc<C>, user_data: Option<Rc<U>>) -> Self
   where
      C: backend::Canvas + 'static,
      U: Any,
   {
      let user_data: Option<Rc<dyn Any>> = user_data.map(|p| {
         let a: Rc<dyn Any> = p;
         a
      });

      Self { inner: canvas, user_data, pen: Pen::default(), brush: Brush::default() }
   }

   #[inline]
   pub fn user_data(&self) -> &Option<Rc<dyn Any>> {
      &self.user_data
   }
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
