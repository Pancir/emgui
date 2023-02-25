use sim_draw::m::{Point2, Rect};
use sim_draw::Canvas;
use std::ops::Range;

////////////////////////////////////////////////////////////////////////////////////////////////////

pub trait Slider1DHandler {
   fn value_changed(&mut self, _value: i32, _range: Range<i32>) {}
}

pub struct DefaultSlider1DHandler {}

impl Slider1DHandler for DefaultSlider1DHandler {}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct Slider1DState {
   pub range: Range<i32>,
   pub value: i32,
   pub rect: Rect<f32>,
   pub handle_rect: Rect<f32>,
   pub handle_position: Point2<f32>,
   pub has_focus: bool,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct Slider1D<PROC = DefaultSlider1DHandler>
where
   PROC: Slider1DHandler,
{
   pub stat: Slider1DState,
   pub handler: PROC,
}

impl Slider1D {
   #[inline]
   pub fn on_draw(&mut self, canvas: &mut Canvas) {

      // canvas.set_color()
   }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
