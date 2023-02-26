use crate::core::events::{LifecycleEvent, MouseButtonsEvent, MouseMoveEvent};
use crate::core::{IWidget, Widget};
use sim_draw::color::Rgba;
use sim_draw::m::{Point2, Rect};
use sim_draw::{Canvas, Paint};
use std::cell::RefCell;
use std::ops::Range;
use std::rc::Rc;

////////////////////////////////////////////////////////////////////////////////////////////////////

pub trait ISlider1DHandler {
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

/// Default Slider handler.
///
/// This implementation uses closures allocated in heap so,
/// in some cases it is better to create you own.
///
/// # Note
/// Heap allocation happens only when you add a closure.
#[derive(Default)]
pub struct Slider1DHandler {
   on_slider_moved: Option<Box<dyn FnMut(&Slider1DState)>>,
   on_range_changed: Option<Box<dyn FnMut(&Slider1DState)>>,
   on_slider_pressed: Option<Box<dyn FnMut(&Slider1DState)>>,
   on_slider_released: Option<Box<dyn FnMut(&Slider1DState)>>,
   on_draw: Option<Box<dyn FnMut(&mut Canvas, &Slider1DState)>>,
}

impl Slider1DHandler {
   /// Construct new.
   pub fn new() -> Self {
      Self::default()
   }

   /// Set callback.
   ///
   /// It allocates memory in heap for the closure.
   pub fn on_slider_moved(mut self, cb: impl FnMut(&Slider1DState) + 'static) -> Self {
      self.on_slider_moved = Some(Box::new(cb));
      self
   }

   /// Set callback.
   ///
   /// It allocates memory in heap for the closure.
   pub fn on_range_changed(mut self, cb: impl FnMut(&Slider1DState) + 'static) -> Self {
      self.on_range_changed = Some(Box::new(cb));
      self
   }

   /// Set callback.
   ///
   /// It allocates memory in heap for the closure.
   pub fn on_slider_pressed(mut self, cb: impl FnMut(&Slider1DState) + 'static) -> Self {
      self.on_slider_pressed = Some(Box::new(cb));
      self
   }

   /// Set callback.
   ///
   /// It allocates memory in heap for the closure.
   pub fn on_slider_released(mut self, cb: impl FnMut(&Slider1DState) + 'static) -> Self {
      self.on_slider_released = Some(Box::new(cb));
      self
   }

   /// Set callback.
   ///
   /// It allocates memory in heap for the closure.
   pub fn on_draw(mut self, cb: impl FnMut(&mut Canvas, &Slider1DState) + 'static) -> Self {
      self.on_draw = Some(Box::new(cb));
      self
   }
}

impl ISlider1DHandler for Slider1DHandler {
   fn slider_moved(&mut self, state: &Slider1DState) {
      if let Some(h) = &mut self.on_slider_moved {
         (h)(state)
      }
   }

   fn range_changed(&mut self, state: &Slider1DState) {
      if let Some(h) = &mut self.on_range_changed {
         (h)(state)
      }
   }

   fn slider_pressed(&mut self, state: &Slider1DState) {
      if let Some(h) = &mut self.on_slider_pressed {
         (h)(state)
      }
   }

   fn slider_released(&mut self, state: &Slider1DState) {
      if let Some(h) = &mut self.on_slider_released {
         (h)(state)
      }
   }

   fn draw(&mut self, canvas: &mut Canvas, state: &Slider1DState) {
      if let Some(h) = &mut self.on_draw {
         (h)(canvas, state)
      } else {
         default_draw(canvas, state)
      }
   }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Default)]
pub struct Slider1DState {
   /// Base data.
   pub base: BaseState,
   //---------------------------------
   /// Value range.
   pub range: Range<i32>,

   /// Current values within the [Self::range].
   pub value: i32,

   /// `True` if horizon orientation, false for vertical.
   pub is_horizon: bool,

   /// Geometry of the handle.
   pub handle_rect: Rect<f32>,

   /// Position of the handle.
   pub handle_position: Point2<f32>,

   /// `True` if mouse pressed over handle.
   pub is_down: bool,

   //---------------------------------
   needs_draw: bool,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct Slider1D<HDL = Slider1DHandler>
where
   HDL: ISlider1DHandler,
{
   pub state: Slider1DState,
   pub handler: HDL,
}

impl<HDL> Slider1D<HDL>
where
   HDL: ISlider1DHandler,
{
   pub fn new(handler: HDL, rect: Rect<f32>) -> Rc<RefCell<Widget<Self>>> {
      let out = Widget::new(|vt| {
         vt.on_draw = Self::on_draw;
         vt.on_mouse_move = Self::on_mouse_move;
         vt.on_mouse_button = Self::on_mouse_button;

         Self {
            handler,
            state: PushButtonState {
               label: Label::new(
                  label,
                  rect.center(),
                  text_patin,
                  TextAlign::new().center().middle(),
               ),
               is_toggled: false,
               is_hover: false,
               is_down: false,
            },
         }
      });

      match out.try_borrow_mut() {
         Ok(mut w) => {
            w.set_rect(rect);
         }
         Err(_) => {
            unreachable!()
         }
      }

      out
   }

   #[inline]
   pub fn with_rect(mut self, rect: Rect<f32>) -> Self {
      self.state.base.geometry.set_rect(rect);
      self
   }

   #[inline]
   pub fn with_range(mut self, range: Range<i32>) -> Self {
      self.state.range = range;
      self
   }

   #[inline]
   pub fn with_value(mut self, value: i32) -> Self {
      self.state.value = value;
      self
   }

   #[inline]
   pub fn disabled(mut self) -> Self {
      self.state.base.is_enabled = false;
      self
   }

   #[inline]
   pub fn visible(mut self) -> Self {
      self.state.base.is_visible = true;
      self
   }
}

impl<HDL> Slider1D<HDL>
where
   HDL: ISlider1DHandler,
{
   #[inline]
   pub fn set_rect(&mut self, rect: Rect<f32>) {
      self.state.base.geometry.set_rect(rect);
   }

   #[inline]
   pub fn set_range(&mut self, range: Range<i32>) {
      self.state.range = range;
   }

   #[inline]
   pub fn set_value(&mut self, value: i32) {
      self.state.value = value;
   }

   #[inline]
   pub fn set_enabled(&mut self, state: bool) {
      self.state.needs_draw = self.state.base.is_enabled != state;
      self.state.base.is_enabled = state;
   }
}

impl<HDL> IWidget for Slider1D<HDL>
where
   HDL: ISlider1DHandler + 'static,
{
   fn base_state(&self) -> &BaseState {
      &self.state.base
   }

   fn set_parent(&mut self, parent: Option<WidgetRef>) {
      self.state.base.parent = parent
   }

   fn set_rect(&mut self, rect: Rect<f32>) {
      self.state.base.geometry.set_rect(rect);
   }

   fn on_lifecycle(&mut self, event: &mut LifecycleEvent) {
      self.state.base.self_ref = event.self_ref.clone()
   }

   fn on_draw(&mut self, canvas: &mut Canvas) {
      self.handler.draw(canvas, &self.state);
   }

   fn on_mouse_move(&mut self, event: &MouseMoveEvent) -> bool {
      let is_over = self.state.base.geometry.rect().is_inside(event.input.x, event.input.y);
      if self.state.base.is_hover != is_over {
         self.request_draw();
      }
      self.state.base.is_hover = is_over;
      self.state.base.is_hover
   }

   fn on_mouse_button(&mut self, _event: &MouseButtonsEvent) -> bool {
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

pub fn default_draw(canvas: &mut Canvas, state: &Slider1DState) {
   canvas.set_paint(Paint::new_color(Rgba::GRAY));
   canvas.fill(&state.base.geometry.rect());
}

////////////////////////////////////////////////////////////////////////////////////////////////////
