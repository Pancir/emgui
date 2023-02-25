use sim_draw::m::{Point2, Rect};
use sim_draw::Canvas;
use sim_input::mouse::{MouseButton, MouseState};
use sim_run::{MouseButtonsEvent, MouseMoveEvent};
use std::ops::Range;

////////////////////////////////////////////////////////////////////////////////////////////////////

pub trait Slider1DHandler {
   /// This is called  when [Slider1DState::is_down] is `true` and the slider moves.
   ///
   /// This usually happens when the user is dragging the slider.
   /// The [Slider1DState::value] is the new slider position.
   fn slider_moved(&mut self, _state: &Slider1DState) {}

   /// This is called  when the slider range has changed.
   ///
   /// The [Slider1DState::range] is the new slider range.
   fn range_changed(&mut self, _state: &Slider1DState) {}

   /// This is called when the user presses the slider with the mouse.
   ///
   /// The [Slider1DState::is_down] is `true`.
   fn slider_pressed(&mut self, _state: &Slider1DState) {}

   /// This is called when the user releases the slider with the mouse.
   ///
   /// The [Slider1DState::is_down] is `false`.
   fn slider_released(&mut self, _state: &Slider1DState) {}

   //----------------------------------------------

   /// This is called when slider should be drown.
   fn draw(&mut self, _canvas: &mut Canvas, _state: &Slider1DState) {}
}

pub struct DefaultSlider1DHandler {}

impl Slider1DHandler for DefaultSlider1DHandler {}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct Slider1DState {
   pub rect: Rect<f32>,

   pub range: Range<i32>,
   pub value: i32,

   pub handle_rect: Rect<f32>,
   pub handle_position: Point2<f32>,

   pub has_focus: bool,
   pub is_down: bool,
   pub is_hover: bool,
   pub is_disabled: bool,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct Slider1D<PROC = DefaultSlider1DHandler>
where
   PROC: Slider1DHandler,
{
   pub state: Slider1DState,
   pub handler: PROC,
}

impl Slider1D {
   pub fn set_disabled(&mut self, state: bool) {
      unimplemented!()
   }
}

impl Slider1D {
   #[inline]
   pub fn on_draw(&mut self, canvas: &mut Canvas) {
      self.handler.draw(canvas, &self.state);
   }

   /// Return `true` if mouse is over.
   #[inline]
   #[must_use]
   pub fn on_mouse_move(&mut self, event: &MouseMoveEvent) -> bool {
      self.state.is_hover = self.state.rect.is_inside(event.input.x, event.input.y);
      self.state.is_hover
   }

   #[inline]
   #[must_use]
   pub fn on_mouse_button(&mut self, event: &MouseButtonsEvent) -> bool {
      // let down =
      //    event.input.state == MouseState::Pressed && event.input.button == MouseButton::Left;
      //
      // let is_click = !down && self.is_hover && self.is_down;
      // self.is_down = down && self.is_hover;
      // if is_click {
      //    self.is_toggle = !self.is_toggle;
      // }
      // is_click
      false
   }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
