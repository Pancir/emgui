use crate::core::derive::Derive;
use crate::core::events::{DrawEventCtx, MouseButtonsEventCtx, MouseMoveEventCtx};
use crate::core::{IWidget, Widget};
use sim_draw::color::Rgba;
use sim_draw::m::{Point2, Rect};
use sim_draw::{Canvas, Paint};
use sim_input::mouse::MouseState;
use std::any::Any;
use std::cell::RefCell;
use std::ops::Range;
use std::rc::Rc;

////////////////////////////////////////////////////////////////////////////////////////////////////

pub trait ISliderHandler {
   /// This is called  when [SliderState::is_down] is `true` and the slider moves.
   ///
   /// This usually happens when the user is dragging the slider.
   /// The [SliderState::value] is the new slider position.
   fn slider_moved(&mut self, _state: &SliderState) {}

   /// This is called  when the slider range has changed.
   ///
   /// The [SliderState::range] is the new slider range.
   fn range_changed(&mut self, _state: &SliderState) {}

   /// This is called when the user presses the slider with the mouse.
   ///
   /// The [SliderState::is_down] is `true`.
   fn slider_pressed(&mut self, _state: &SliderState) {}

   /// This is called when the user releases the slider with the mouse.
   ///
   /// The [SliderState::is_down] is `false`.
   fn slider_released(&mut self, _state: &SliderState) {}
}

/// Default Slider handler.
///
/// This implementation uses closures allocated in heap so,
/// in some cases it is better to create you own.
///
/// # Note
/// Heap allocation happens only when you add a closure.
#[derive(Default)]
pub struct SliderHandler {
   on_slider_moved: Option<Box<dyn FnMut(&SliderState)>>,
   on_range_changed: Option<Box<dyn FnMut(&SliderState)>>,
   on_slider_pressed: Option<Box<dyn FnMut(&SliderState)>>,
   on_slider_released: Option<Box<dyn FnMut(&SliderState)>>,
   on_draw: Option<Box<dyn FnMut(&mut Canvas, &SliderState)>>,
}

impl SliderHandler {
   /// Construct new.
   pub fn new() -> Self {
      Self::default()
   }

   /// Set callback.
   ///
   /// It allocates memory in heap for the closure.
   pub fn on_slider_moved(mut self, cb: impl FnMut(&SliderState) + 'static) -> Self {
      self.on_slider_moved = Some(Box::new(cb));
      self
   }

   /// Set callback.
   ///
   /// It allocates memory in heap for the closure.
   pub fn on_range_changed(mut self, cb: impl FnMut(&SliderState) + 'static) -> Self {
      self.on_range_changed = Some(Box::new(cb));
      self
   }

   /// Set callback.
   ///
   /// It allocates memory in heap for the closure.
   pub fn on_slider_pressed(mut self, cb: impl FnMut(&SliderState) + 'static) -> Self {
      self.on_slider_pressed = Some(Box::new(cb));
      self
   }

   /// Set callback.
   ///
   /// It allocates memory in heap for the closure.
   pub fn on_slider_released(mut self, cb: impl FnMut(&SliderState) + 'static) -> Self {
      self.on_slider_released = Some(Box::new(cb));
      self
   }
}

impl ISliderHandler for SliderHandler {
   fn slider_moved(&mut self, state: &SliderState) {
      if let Some(h) = &mut self.on_slider_moved {
         (h)(state)
      }
   }

   fn range_changed(&mut self, state: &SliderState) {
      if let Some(h) = &mut self.on_range_changed {
         (h)(state)
      }
   }

   fn slider_pressed(&mut self, state: &SliderState) {
      if let Some(h) = &mut self.on_slider_pressed {
         (h)(state)
      }
   }

   fn slider_released(&mut self, state: &SliderState) {
      if let Some(h) = &mut self.on_slider_released {
         (h)(state)
      }
   }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Default)]
pub struct SliderState {
   /// Value range.
   pub range: Range<i32>,

   /// Current values within the [Self::range].
   pub value: i32,

   /// `True` if horizon orientation, false for vertical.
   pub is_vertical: bool,

   /// Geometry of the handle.
   pub handle_rect: Rect<f32>,

   /// Position of the handle.
   pub handle_position: Point2<f32>,

   /// `True` if mouse pressed over handle.
   pub is_down: bool,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct Slider<H = SliderHandler>
where
   H: ISliderHandler,
{
   state: SliderState,
   handler: H,
}

impl<H> Derive for Slider<H>
where
   H: ISliderHandler + 'static,
{
   fn as_any(&self) -> &dyn Any {
      self
   }

   fn as_any_mut(&mut self) -> &mut dyn Any {
      self
   }
}

impl<H> Slider<H>
where
   H: ISliderHandler + 'static,
{
   pub fn new(handler: H, rect: Rect<f32>) -> Rc<RefCell<Widget<Self>>> {
      Widget::new(
         |vt| {
            vt.on_draw = Self::on_draw;
            vt.on_mouse_enter = Self::on_mouse_enter;
            vt.on_mouse_leave = Self::on_mouse_leave;
            vt.on_mouse_button = Self::on_mouse_button;

            Self {
               handler,
               state: SliderState {
                  range: Range { start: 0, end: 100 },
                  value: 0,
                  is_vertical: false,
                  handle_rect: Default::default(),
                  handle_position: Default::default(),
                  is_down: false,
               },
            }
         },
         |w| {
            w.set_rect(rect);
         },
      )
   }
}

impl<H> Slider<H>
where
   H: ISliderHandler + 'static,
{
   fn on_draw(w: &mut Widget<Self>, canvas: &mut Canvas, _event: &DrawEventCtx) {
      // let d = w.derive_ref();
      //
      // canvas.set_paint(Paint::new_color(Rgba::GREEN.with_alpha_mul(0.5)));
      //
      // if d.state.is_hover {
      //    canvas.set_color(Rgba::GRAY);
      // }
      //
      // if d.state.is_down {
      //    canvas.set_color(Rgba::GRAY_LIGHT);
      // }
      //
      // canvas.fill(&w.geometry().rect());
      // if !d.state.label.text.is_empty() {
      //    d.state.label.on_draw(canvas);
      // }
   }

   pub fn on_mouse_enter(w: &mut Widget<Self>) {
      // let mut d = w.derive_mut();
      // d.state.is_hover = true;
      // w.request_draw();
   }

   pub fn on_mouse_leave(w: &mut Widget<Self>) {
      // let mut d = w.derive_mut();
      // d.state.is_hover = false;
      // w.request_draw();
   }

   pub fn on_mouse_button(w: &mut Widget<Self>, event: &MouseButtonsEventCtx) -> bool {
      let mut d = w.derive_mut();

      // match event.input.state {
      //    MouseState::Pressed => {
      //       if d.state.is_hover {
      //          d.state.is_down = true;
      //          d.handler.pressed(&d.state, event.input.button);
      //          w.request_draw();
      //          return true;
      //       }
      //    }
      //    MouseState::Released => {
      //       if d.state.is_down {
      //          d.state.is_down = false;
      //          d.handler.released(&d.state, event.input.button);
      //
      //          if d.state.is_hover {
      //             d.state.toggle += 1;
      //             if d.state.toggle == d.state.toggle_num {
      //                d.state.toggle = 0;
      //             }
      //
      //             d.handler.click(&d.state);
      //          }
      //          w.request_draw();
      //          return true;
      //       }
      //    }
      // }

      false
   }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
