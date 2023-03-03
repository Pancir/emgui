use crate::core::derive::Derive;
use crate::core::events::{DrawEventCtx, MouseButtonsEventCtx, MouseMoveEventCtx};
use crate::core::{IWidget, Widget};
use sim_draw::color::Rgba;
use sim_draw::m::{Box2, Point2, Rect};
use sim_draw::{Canvas, Paint};
use sim_input::mouse::{MouseButton, MouseState};
use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

////////////////////////////////////////////////////////////////////////////////////////////////////

pub trait ISlider2dHandler {
   /// This is called  when [Slider2dState::is_down] is `true` and the slider moves.
   ///
   /// This usually happens when the user is dragging the slider.
   /// The [Slider2dState::value] is the new slider position.
   fn slider_moved(&mut self, _state: &Slider2dState) {}

   /// This is called  when the slider range has changed.
   ///
   /// The [Slider2dState::range] is the new slider range.
   fn range_changed(&mut self, _state: &Slider2dState) {}

   /// This is called when the user presses the slider with the mouse.
   ///
   /// The [Slider2dState::is_down] is `true`.
   fn slider_pressed(&mut self, _state: &Slider2dState) {}

   /// This is called when the user releases the slider with the mouse.
   ///
   /// The [Slider2dState::is_down] is `false`.
   fn slider_released(&mut self, _state: &Slider2dState) {}
}

/// Default Slider handler.
///
/// This implementation uses closures allocated in heap so,
/// in some cases it is better to create you own.
///
/// # Note
/// Heap allocation happens only when you add a closure.
#[derive(Default)]
pub struct Slider2dHandler {
   on_slider_moved: Option<Box<dyn FnMut(&Slider2dState)>>,
   on_range_changed: Option<Box<dyn FnMut(&Slider2dState)>>,
   on_slider_pressed: Option<Box<dyn FnMut(&Slider2dState)>>,
   on_slider_released: Option<Box<dyn FnMut(&Slider2dState)>>,
   on_draw: Option<Box<dyn FnMut(&mut Canvas, &Slider2dState)>>,
}

impl Slider2dHandler {
   /// Construct new.
   pub fn new() -> Self {
      Self::default()
   }

   /// Set callback.
   ///
   /// It allocates memory in heap for the closure.
   pub fn on_slider_moved(mut self, cb: impl FnMut(&Slider2dState) + 'static) -> Self {
      self.on_slider_moved = Some(Box::new(cb));
      self
   }

   /// Set callback.
   ///
   /// It allocates memory in heap for the closure.
   pub fn on_range_changed(mut self, cb: impl FnMut(&Slider2dState) + 'static) -> Self {
      self.on_range_changed = Some(Box::new(cb));
      self
   }

   /// Set callback.
   ///
   /// It allocates memory in heap for the closure.
   pub fn on_slider_pressed(mut self, cb: impl FnMut(&Slider2dState) + 'static) -> Self {
      self.on_slider_pressed = Some(Box::new(cb));
      self
   }

   /// Set callback.
   ///
   /// It allocates memory in heap for the closure.
   pub fn on_slider_released(mut self, cb: impl FnMut(&Slider2dState) + 'static) -> Self {
      self.on_slider_released = Some(Box::new(cb));
      self
   }
}

impl ISlider2dHandler for Slider2dHandler {
   fn slider_moved(&mut self, state: &Slider2dState) {
      if let Some(h) = &mut self.on_slider_moved {
         (h)(state)
      }
   }

   fn range_changed(&mut self, state: &Slider2dState) {
      if let Some(h) = &mut self.on_range_changed {
         (h)(state)
      }
   }

   fn slider_pressed(&mut self, state: &Slider2dState) {
      if let Some(h) = &mut self.on_slider_pressed {
         (h)(state)
      }
   }

   fn slider_released(&mut self, state: &Slider2dState) {
      if let Some(h) = &mut self.on_slider_released {
         (h)(state)
      }
   }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Default)]
pub struct Slider2dState {
   /// Current X value from `0.0..=1.0`
   pub value_x: f32,

   /// Current Y value from `0.0..=1.0`
   pub value_y: f32,

   //---------------------------
   /// Mouse pressed on handle.
   pub is_down: bool,

   /// Grab mouse position area, it is usually less than the
   /// widget rectangle, because it need a room for handle draw.
   pub grab_area: Box2<f32>,

   /// Geometry of the handle relative to the [Self::handle_position].
   pub handle_rect: Rect<f32>,

   /// Position of the handle.
   pub handle_position: Point2<f32>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct Slider2d<H = Slider2dHandler>
where
   H: ISlider2dHandler,
{
   state: Slider2dState,
   handler: H,

   click_pos: Point2<f32>,
}

impl<H> Derive for Slider2d<H>
where
   H: ISlider2dHandler + 'static,
{
   fn as_any(&self) -> &dyn Any {
      self
   }

   fn as_any_mut(&mut self) -> &mut dyn Any {
      self
   }
}

impl<H> Slider2d<H>
where
   H: ISlider2dHandler + 'static,
{
   pub fn new(handler: H, rect: Rect<f32>) -> Rc<RefCell<Widget<Self>>> {
      Widget::new(
         |vt| {
            vt.on_draw = Self::on_draw;
            vt.on_set_rect = Self::on_set_rect;
            vt.on_mouse_move = Self::on_mouse_move;
            vt.on_mouse_button = Self::on_mouse_button;

            let handle_rect = Rect::new(-15.0, -15.0, 30.0, 30.0);

            Self {
               handler,
               click_pos: Point2::ZERO,
               state: Slider2dState {
                  value_x: 0.5,
                  value_y: 0.5,
                  is_down: false,
                  grab_area: Box2::ZERO,
                  handle_rect,
                  handle_position: -handle_rect.pos() + rect.pos(),
               },
            }
         },
         |w| {
            w.set_rect(rect);
         },
      )
   }
}

impl<H> Slider2d<H>
where
   H: ISlider2dHandler + 'static,
{
   fn on_set_rect(w: &mut Widget<Self>, rect: Rect<f32>) -> Option<Rect<f32>> {
      let d = w.derive_mut();
      let l_off = d.state.handle_rect.x;
      let r_off = d.state.handle_rect.width + d.state.handle_rect.x;
      let t_off = d.state.handle_rect.y;
      let b_off = d.state.handle_rect.height + d.state.handle_rect.y;
      d.state.grab_area = rect.margin(l_off, -r_off, t_off, -b_off).into();

      Some(rect)
   }

   fn on_draw(w: &mut Widget<Self>, canvas: &mut Canvas, _event: &DrawEventCtx) {
      let d = w.derive_ref();
      let rect = &w.geometry().rect();

      canvas.set_paint(Paint::new_color(Rgba::AMBER));
      canvas.fill(&w.geometry().rect());

      canvas.set_paint(Paint::new_color(Rgba::GRAY));
      canvas.fill(&d.state.grab_area);

      // let w_pos = d.state.handle_position + rect.pos();
      let h_rect = d.state.handle_rect.offset(d.state.handle_position);
      if d.state.is_down {
         canvas.set_paint(Paint::new_color(Rgba::CYAN));
      } else {
         canvas.set_paint(Paint::new_color(Rgba::GREEN));
      }
      canvas.fill(&h_rect);
   }

   pub fn on_mouse_move(w: &mut Widget<Self>, event: &MouseMoveEventCtx) -> bool {
      // let rect = &w.geometry().rect();
      let mut d = w.derive_mut();

      let mouse_pos = Point2::new(
         event.input.x.clamp(d.state.grab_area.min.x, d.state.grab_area.max.x),
         event.input.y.clamp(d.state.grab_area.min.y, d.state.grab_area.max.y),
      );

      d.state.handle_position = mouse_pos - d.click_pos;
      w.request_draw();
      true
   }

   pub fn on_mouse_button(w: &mut Widget<Self>, event: &MouseButtonsEventCtx) -> bool {
      let rect = &w.geometry().rect();
      let mut d = w.derive_mut();
      let h_rect = d.state.handle_rect.offset(d.state.handle_position);

      if event.input.button != MouseButton::Left || !rect.is_inside(event.input.x, event.input.y) {
         return true;
      }

      match event.input.state {
         MouseState::Pressed => {
            d.click_pos = Point2::new(event.input.x, event.input.y) - d.state.handle_position;
            d.state.is_down = true;
            w.request_draw();
         }
         MouseState::Released => {
            d.state.is_down = false;
            w.request_draw();
         }
      }

      true
   }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
