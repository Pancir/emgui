use crate::core::events::{DrawEventCtx, MouseButtonsEventCtx, MouseMoveEventCtx};
use crate::core::{IWidget, WidgetRefOwner};
use crate::widgets::Widget;
use sim_draw::color::Rgba;
use sim_draw::m::{Box2, Point2, Rect};
use sim_draw::{Canvas, Paint};
use sim_input::mouse::{MouseButton, MouseState};
use std::cell::RefCell;
use std::ops::Range;
use std::rc::Rc;
use std::time::Instant;

////////////////////////////////////////////////////////////////////////////////////////////////////

/// Slider value with range.
#[derive(Clone)]
pub struct SliderValue {
   pub range: Range<i32>,
   pub value: i32,
}

impl Default for SliderValue {
   fn default() -> Self {
      Self { range: Range { start: 0, end: 100 }, value: 0 }
   }
}

impl SliderValue {
   /// Convert value to range of `0.0..=1.0`.
   #[inline]
   pub fn unit_interval(&self) -> f32 {
      debug_assert!(self.range.start != self.range.end, "Range can't be 0");
      (self.value - self.range.start) as f32 / (self.range.end - self.range.start) as f32
   }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub trait ISlider2dHandler {
   /// This is called  when [Slider2dState::is_down] is `true` and the slider moves.
   ///
   /// This usually happens when the user is dragging the slider.
   fn slider_moved(&mut self, _x: SliderValue, _y: SliderValue) {}

   /// This is called when the user presses the slider with the mouse.
   ///
   /// The [Slider2dState::is_down] is `true`.
   fn slider_pressed(&mut self, _x: SliderValue, _y: SliderValue) {}

   /// This is called when the user releases the slider with the mouse.
   ///
   /// The [Slider2dState::is_down] is `false`.
   fn slider_released(&mut self, _x: SliderValue, _y: SliderValue) {}
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
   on_slider_moved: Option<Box<dyn FnMut(SliderValue, SliderValue)>>,
   on_slider_pressed: Option<Box<dyn FnMut(SliderValue, SliderValue)>>,
   on_slider_released: Option<Box<dyn FnMut(SliderValue, SliderValue)>>,
}

impl Slider2dHandler {
   /// Construct new.
   pub fn new() -> Self {
      Self::default()
   }

   /// Set callback.
   ///
   /// It allocates memory in heap for the closure.
   pub fn on_slider_moved(mut self, cb: impl FnMut(SliderValue, SliderValue) + 'static) -> Self {
      self.on_slider_moved = Some(Box::new(cb));
      self
   }
   /// Set callback.
   ///
   /// It allocates memory in heap for the closure.
   pub fn on_slider_pressed(mut self, cb: impl FnMut(SliderValue, SliderValue) + 'static) -> Self {
      self.on_slider_pressed = Some(Box::new(cb));
      self
   }

   /// Set callback.
   ///
   /// It allocates memory in heap for the closure.
   pub fn on_slider_released(mut self, cb: impl FnMut(SliderValue, SliderValue) + 'static) -> Self {
      self.on_slider_released = Some(Box::new(cb));
      self
   }
}

impl ISlider2dHandler for Slider2dHandler {
   fn slider_moved(&mut self, x: SliderValue, y: SliderValue) {
      if let Some(h) = &mut self.on_slider_moved {
         (h)(x, y)
      }
   }

   fn slider_pressed(&mut self, x: SliderValue, y: SliderValue) {
      if let Some(h) = &mut self.on_slider_pressed {
         (h)(x, y)
      }
   }

   fn slider_released(&mut self, x: SliderValue, y: SliderValue) {
      if let Some(h) = &mut self.on_slider_released {
         (h)(x, y)
      }
   }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Default)]
pub struct Slider2dState {
   /// X value.
   pub x: SliderValue,

   /// Y value.
   pub y: SliderValue,

   /// Default [Self::x] that is used when the slider is reset.
   pub default_x: i32,

   /// Default [Self::y] that is used when the slider is reset.
   pub default_y: i32,

   //---------------------------
   /// Mouse pressed on handle.
   pub is_handle_down: bool,

   /// Mouse is over handle.
   pub is_over_handle: bool,

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
   released_at: Instant,
}

impl<H> Slider2d<H>
where
   H: ISlider2dHandler + 'static,
{
   pub fn new(handler: H, rect: Rect<f32>) -> WidgetRefOwner {
      Self::new_flat(handler, rect).to_owner()
   }

   pub fn new_flat(handler: H, rect: Rect<f32>) -> Widget<Self> {
      Widget::inherit(
         |vt| {
            vt.on_draw = Self::on_draw;
            vt.on_set_rect = Self::on_set_rect;
            vt.on_mouse_move = Self::on_mouse_move;
            vt.on_mouse_button = Self::on_mouse_button;

            let handle_rect = Rect::new(-15.0, -15.0, 30.0, 30.0);

            Self {
               handler,
               click_pos: Point2::ZERO,
               released_at: Instant::now(),
               state: Slider2dState {
                  x: SliderValue::default(),
                  y: SliderValue::default(),
                  default_x: 50,
                  default_y: 50,
                  is_handle_down: false,
                  is_over_handle: false,
                  grab_area: Box2::ZERO,
                  handle_rect,
                  handle_position: -handle_rect.pos() + rect.pos(),
               },
            }
         },
         |w| {
            w.base().set_mouse_tracking(true);
            w.set_rect(rect);
         },
      )
   }

   pub fn to_rc(self) -> Rc<RefCell<Self>> {
      Rc::new(RefCell::new(self))
   }

   //----------------------------------------------------------

   #[inline]
   pub fn set(&mut self, r_x: Range<i32>, r_y: Range<i32>, v_x: i32, v_y: i32, d_x: i32, d_y: i32) {
      self.set_ranges(r_x, r_y);
      self.set_values(v_x, v_y);
      self.set_default_values(d_x, d_y);
   }

   //----------------------------------------------------------

   #[inline]
   pub fn set_ranges(&mut self, x: Range<i32>, y: Range<i32>) {
      self.set_x_range(x);
      self.set_y_range(y);
   }

   #[inline]
   pub fn set_x_range(&mut self, range: Range<i32>) {
      self.state.x.range = range;
      self.state.x.value =
         self.state.x.value.clamp(self.state.x.range.start, self.state.x.range.end);
   }

   #[inline]
   pub fn set_y_range(&mut self, range: Range<i32>) {
      self.state.y.range = range;
      self.state.y.value =
         self.state.y.value.clamp(self.state.y.range.start, self.state.y.range.end);
   }

   #[inline]
   pub fn ranges(&self) -> (Range<i32>, Range<i32>) {
      (self.state.x.range.clone(), self.state.y.range.clone())
   }

   //----------------------------------------------------------

   #[inline]
   pub fn set_values(&mut self, x: i32, y: i32) {
      self.set_x_value(x);
      self.set_y_value(y);
   }

   #[inline]
   pub fn set_x_value(&mut self, value: i32) {
      self.state.x.value = value.clamp(self.state.x.range.start, self.state.x.range.end);
      self.set_handle_position_x(self.state.x.unit_interval());
   }

   #[inline]
   pub fn set_y_value(&mut self, value: i32) {
      self.state.y.value = value.clamp(self.state.y.range.start, self.state.y.range.end);
      self.set_handle_position_y(self.state.y.unit_interval());
   }

   #[inline]
   pub fn values(&self) -> (i32, i32) {
      (self.state.x.value, self.state.y.value)
   }

   //----------------------------------------------------------

   #[inline]
   pub fn set_default_values(&mut self, x: i32, y: i32) {
      self.set_default_x_value(x);
      self.set_default_y_value(y);
   }

   #[inline]
   pub fn set_default_x_value(&mut self, value: i32) {
      self.state.default_x = value;
   }

   #[inline]
   pub fn set_default_y_value(&mut self, value: i32) {
      self.state.default_y = value;
   }

   #[inline]
   pub fn default_values(&self) -> (i32, i32) {
      (self.state.default_x, self.state.default_y)
   }
}

impl<H> Slider2d<H>
where
   H: ISlider2dHandler + 'static,
{
   fn map_to_unit(start: f32, end: f32, v: f32) -> f32 {
      (v - start) / (end - start)
   }

   fn get_handle_position(&self) -> (f32, f32) {
      let width = self.state.grab_area.width();
      let height = self.state.grab_area.height();

      let x = if width > 0.0 {
         Self::map_to_unit(
            self.state.grab_area.min.x,
            self.state.grab_area.max.x,
            self.state.handle_position.x,
         )
      } else {
         0.0
      };

      let y = if height > 0.0 {
         Self::map_to_unit(
            self.state.grab_area.min.y,
            self.state.grab_area.max.y,
            self.state.handle_position.y,
         )
      } else {
         0.0
      };

      (x, y)
   }

   fn set_handle_position_x(&mut self, x: f32) {
      debug_assert!((0.0..=1.0).contains(&x));
      self.state.handle_position.x = self.state.grab_area.min.x + self.state.grab_area.width() * x;
   }

   fn set_handle_position_y(&mut self, y: f32) {
      debug_assert!((0.0..=1.0).contains(&y));
      self.state.handle_position.y = self.state.grab_area.min.y + self.state.grab_area.height() * y;
   }

   fn set_handle_position(&mut self, x: f32, y: f32) {
      self.set_handle_position_x(x);
      self.set_handle_position_y(y);
   }

   fn set_handle_value_to_values(&mut self, v: (f32, f32)) {
      debug_assert!(self.state.x.range.start != self.state.x.range.end, "range can't be 0");
      debug_assert!(self.state.y.range.start != self.state.y.range.end, "range can't be 0");

      self.state.x.value = ((self.state.x.range.end - self.state.x.range.start) as f32 * v.0)
         as i32
         + self.state.x.range.start;

      self.state.y.value = ((self.state.y.range.end - self.state.y.range.start) as f32 * v.1)
         as i32
         + self.state.y.range.start;
   }

   fn reset_handle_position(&mut self) {
      // Default can't be out of range.

      let default_x = self.state.default_x.clamp(self.state.x.range.start, self.state.x.range.end);
      let default_y = self.state.default_y.clamp(self.state.y.range.start, self.state.y.range.end);

      let x = SliderValue { range: self.state.x.range.clone(), value: default_x };
      let y = SliderValue { range: self.state.y.range.clone(), value: default_y };

      self.set_handle_position(x.unit_interval(), y.unit_interval())
   }
}

impl<H> Slider2d<H>
where
   H: ISlider2dHandler + 'static,
{
   fn on_set_rect(w: &mut Widget<Self>, rect: Rect<f32>) -> Option<Rect<f32>> {
      let d = w.inherited_obj_mut();
      let l_off = d.state.handle_rect.x;
      let r_off = d.state.handle_rect.width + d.state.handle_rect.x;
      let t_off = d.state.handle_rect.y;
      let b_off = d.state.handle_rect.height + d.state.handle_rect.y;
      d.state.grab_area = rect.margin(l_off, -r_off, t_off, -b_off).into();
      d.set_handle_position(d.state.x.unit_interval(), d.state.y.unit_interval());

      Some(rect)
   }

   fn on_draw(w: &mut Widget<Self>, canvas: &mut Canvas, _event: &DrawEventCtx) {
      let d = w.inherited_obj();
      let rect = w.base().geometry().rect();

      canvas.set_paint(Paint::new_color(Rgba::AMBER));
      canvas.fill(&rect);

      canvas.set_paint(Paint::new_color(Rgba::GRAY));
      canvas.fill(&d.state.grab_area);

      // let w_pos = d.state.handle_position + rect.pos();
      let h_rect = d.state.handle_rect.offset(d.state.handle_position);
      if d.state.is_handle_down {
         canvas.set_paint(Paint::new_color(Rgba::CYAN));
      } else if d.state.is_over_handle {
         canvas.set_paint(Paint::new_color(Rgba::GREEN.with_alpha_mul(0.9)));
      } else {
         canvas.set_paint(Paint::new_color(Rgba::GREEN));
      }
      canvas.fill(&h_rect);
   }

   pub fn on_mouse_move(w: &mut Widget<Self>, event: &MouseMoveEventCtx) -> bool {
      let mut d = w.inherited_obj_mut();

      if d.state.is_handle_down {
         // TODO Snap vert/hor.
         // if event.input.modifiers.shift() {
         let mouse_pos = Point2::new(event.input.x, event.input.y);

         d.state.handle_position =
            (mouse_pos - d.click_pos).clamp(d.state.grab_area.min, d.state.grab_area.max);

         d.set_handle_value_to_values(d.get_handle_position());
         d.handler.slider_moved(d.state.x.clone(), d.state.y.clone());

         w.base().request_draw();
         // }
      } else {
         let h_rect = d.state.handle_rect.offset(d.state.handle_position);
         let is_inside = h_rect.is_inside(event.input.x, event.input.y);

         if is_inside != d.state.is_over_handle {
            d.state.is_over_handle = is_inside;
            w.base().request_draw();
         }
      }
      true
   }

   pub fn on_mouse_button(w: &mut Widget<Self>, event: &MouseButtonsEventCtx) -> bool {
      if event.input.button != MouseButton::Left {
         return true;
      }

      let is_inside_handle = {
         let d = w.inherited_obj();
         let h_rect = d.state.handle_rect.offset(d.state.handle_position);
         h_rect.is_inside(event.input.x, event.input.y)
      };

      match event.input.state {
         MouseState::Pressed => {
            if is_inside_handle {
               let mut d = w.inherited_obj_mut();
               d.click_pos = Point2::new(event.input.x, event.input.y) - d.state.handle_position;
               d.state.is_handle_down = true;
               d.handler.slider_pressed(d.state.x.clone(), d.state.y.clone());
               w.base().request_draw();
            }
         }
         MouseState::Released => {
            let mut d = w.inherited_obj_mut();
            if d.state.is_handle_down {
               d.state.is_handle_down = false;
               d.handler.slider_released(d.state.x.clone(), d.state.y.clone());

               if is_inside_handle {
                  let db_time = w.base().double_click_time();
                  let d = w.inherited_obj_mut();

                  if d.released_at.elapsed() < db_time {
                     d.reset_handle_position();

                     d.set_handle_value_to_values(d.get_handle_position());
                     d.handler.slider_moved(d.state.x.clone(), d.state.y.clone());
                  }
               }

               let mut d = w.inherited_obj_mut();
               d.released_at = Instant::now();
               w.base().request_draw();
            }
         }
      }

      true
   }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
