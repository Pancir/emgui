pub mod handler;
pub mod style;

use self::style::ButtonStyleSheet;
use super::handler::IButtonHandler;
use crate::core::events::{
   DrawEventCtx, KeyboardEventCtx, LayoutEventCtx, LifecycleEventCtx, MouseButtonsEventCtx,
   MouseMoveEventCtx, MouseWheelEventCtx, UpdateEventCtx,
};
use crate::core::{IWidget, WidgetBase, WidgetVt};
use crate::elements::Icon;
use sim_draw::{color::Rgba, m::Rect};
use sim_draw::{Canvas, Paint};
use sim_input::mouse::MouseState;
use std::borrow::Cow;
use std::rc::Rc;
use std::{any::Any, mem::MaybeUninit};

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Default)]
pub struct ButtonState {
   pub toggle_num: u8,
   pub toggle: u8,
   pub is_hover: bool,
   pub is_down: bool,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct Button<H, D>
where
   H: IButtonHandler,
   D: Any,
{
   base: WidgetBase,
   inherited: MaybeUninit<D>,
   vtable: WidgetVt<Self>,
   handler: H,

   style: Option<Rc<dyn ButtonStyleSheet>>,
   state: ButtonState,
   text: Option<Cow<'static, str>>,
   icon: Option<Icon>,
}

impl<H> Button<H, ()>
where
   H: IButtonHandler + 'static,
{
   /// Construct new button.
   pub fn new<T>(handler: H, text: Option<T>, icon: Option<Icon>) -> Self
   where
      T: Into<Cow<'static, str>>,
   {
      let mut out = Self::inherit(|_| (), |_| {}, handler);
      out.set_text(text);
      out.set_icon(icon);
      out
   }
}

impl<H, D> Button<H, D>
where
   H: IButtonHandler + 'static,
   D: Any,
{
   /// Construct a derived entity.
   pub fn inherit<VCB, ICB>(vt_cb: VCB, init_cb: ICB, handler: H) -> Self
   where
      VCB: FnOnce(&mut WidgetVt<Self>) -> D,
      ICB: FnOnce(&mut Self),
   {
      let mut out = Self {
         base: WidgetBase::new::<Self>(),
         inherited: unsafe { std::mem::zeroed() },
         vtable: WidgetVt {
            on_draw: Self::on_draw,
            on_mouse_enter: Self::on_mouse_enter,
            on_mouse_leave: Self::on_mouse_leave,
            on_mouse_button: Self::on_mouse_button,
            ..WidgetVt::default()
         },
         handler,
         style: None,
         state: ButtonState::default(),
         text: None,
         icon: None,
      };

      let inherited = vt_cb(&mut out.vtable);
      out.inherited.write(inherited);

      init_cb(&mut out);
      out
   }

   /// Set button text.
   #[inline]
   pub fn set_icon(&mut self, icon: Option<Icon>) {
      self.icon = icon
   }

   /// Set button text.
   #[inline]
   pub fn set_text<T>(&mut self, text: Option<T>)
   where
      T: Into<Cow<'static, str>>,
   {
      self.text = text.map(|v| v.into())
   }

   /// Access to button's derive object.
   #[inline]
   pub fn inherited_obj(&self) -> &D {
      // # Safety
      // All initialization happen in a constructor.
      unsafe { self.inherited.assume_init_ref() }
   }

   /// Mut access to button's derive object.
   #[inline]
   pub fn inherited_obj_mut(&mut self) -> &mut D {
      // # Safety
      // All initialization happen in a constructor.
      unsafe { self.inherited.assume_init_mut() }
   }

   /// Replace button's handler with a new one.
   #[inline]
   pub fn set_handler(&mut self, h: H) {
      self.handler = h
   }

   /// Access to button's handler.
   #[inline]
   pub fn handler(&self) -> &H {
      &self.handler
   }

   /// Mut access to button's handler.
   #[inline]
   pub fn handler_mut(&mut self) -> &mut H {
      &mut self.handler
   }

   /// Access to button's state.
   #[inline]
   pub fn state(&self) -> &ButtonState {
      &self.state
   }
}

impl<H, D: 'static> IWidget for Button<H, D>
where
   H: IButtonHandler + 'static,
   D: Any,
{
   fn as_any(&self) -> &dyn Any {
      self
   }

   fn as_any_mut(&mut self) -> &mut dyn Any {
      self
   }

   //---------------------------------------

   fn base(&self) -> &WidgetBase {
      &self.base
   }

   fn base_mut(&mut self) -> &mut WidgetBase {
      &mut self.base
   }

   fn inherited(&self) -> &dyn Any {
      self.inherited_obj()
   }

   fn inherited_mut(&mut self) -> &mut dyn Any {
      self.inherited_obj_mut()
   }

   //---------------------------------------

   fn set_rect(&mut self, r: Rect<f32>) {
      if let Some(rect) = (self.vtable.on_set_rect)(self, r) {
         self.base.geometry_mut().set_rect(rect);
      }
   }

   //---------------------------------------

   fn set_visible(&mut self, state: bool) {
      if self.base.set_visible(state) {
         (self.vtable.on_visible)(self, state)
      }
   }

   fn set_enabled(&mut self, state: bool) {
      if self.base.set_enabled(state) {
         (self.vtable.on_disable)(self, !state)
      }
   }

   fn set_transparent(&mut self, state: bool) {
      self.base.set_transparent(state)
   }

   //---------------------------------------

   #[cfg_attr(feature = "trace-widget",
   tracing::instrument(skip(self, event), fields(WidgetID = self.base().id().raw(), ret)))]
   fn on_lifecycle(&mut self, event: &LifecycleEventCtx) {
      (self.vtable.on_lifecycle)(self, event);
   }

   #[cfg_attr(feature = "trace-widget",
   tracing::instrument(skip(self, event), fields(WidgetID = self.base().id().raw(), ret)))]
   fn on_layout(&mut self, event: &LayoutEventCtx) {
      (self.vtable.on_layout)(self, event);
   }

   #[cfg_attr(feature = "trace-widget",
   tracing::instrument(skip(self, canvas, event), fields(WidgetID = self.base().id().raw(), ret)))]
   fn on_draw(&mut self, canvas: &mut Canvas, event: &DrawEventCtx) {
      (self.vtable.on_draw)(self, canvas, event);
   }

   #[cfg_attr(feature = "trace-widget",
   tracing::instrument(skip(self, event), fields(WidgetID = self.base().id().raw(), ret)))]
   fn on_update(&mut self, event: &UpdateEventCtx) {
      (self.vtable.on_update)(self, event);
   }

   #[cfg_attr(feature = "trace-widget",
   tracing::instrument(skip(self), fields(WidgetID = self.base().id().raw()), ret))]
   fn on_mouse_enter(&mut self) {
      (self.vtable.on_mouse_enter)(self)
   }

   #[cfg_attr(feature = "trace-widget",
   tracing::instrument(skip(self), fields(WidgetID = self.base().id().raw()), ret))]
   fn on_mouse_leave(&mut self) {
      (self.vtable.on_mouse_leave)(self)
   }

   #[must_use]
   #[cfg_attr(feature = "trace-widget",
   tracing::instrument(skip(self, event), fields(WidgetID = self.base().id().raw()), ret))]
   fn on_mouse_move(&mut self, event: &MouseMoveEventCtx) -> bool {
      (self.vtable.on_mouse_move)(self, event)
   }

   #[must_use]
   #[cfg_attr(feature = "trace-widget",
   tracing::instrument(skip(self, event), fields(WidgetID = self.base().id().raw()), ret))]
   fn on_mouse_button(&mut self, event: &MouseButtonsEventCtx) -> bool {
      (self.vtable.on_mouse_button)(self, event)
   }

   #[must_use]
   #[cfg_attr(feature = "trace-widget",
   tracing::instrument(skip(self, event), fields(WidgetID = self.base().id().raw()), ret))]
   fn on_mouse_wheel(&mut self, event: &MouseWheelEventCtx) -> bool {
      (self.vtable.on_mouse_wheel)(self, event)
   }

   #[must_use]
   #[cfg_attr(feature = "trace-widget",
   tracing::instrument(skip(self, event), fields(WidgetID = self.base().id().raw()), ret))]
   fn on_keyboard(&mut self, event: &KeyboardEventCtx) -> bool {
      (self.vtable.on_keyboard)(self, event)
   }
}

impl<H, D> Button<H, D>
where
   H: IButtonHandler + 'static,
   D: Any,
{
   fn on_draw(w: &mut Self, canvas: &mut Canvas, _event: &DrawEventCtx) {
      canvas.set_paint(Paint::new_color(Rgba::GREEN.with_alpha_mul(0.5)));

      if w.state.is_hover {
         canvas.set_color(Rgba::AMBER);
      }

      if w.state.is_down {
         canvas.set_color(Rgba::RED);
      }

      let rect = w.base.geometry().rect();

      canvas.fill(&rect);

      canvas.set_color(Rgba::BLACK);
      canvas.set_aa_fringe(Some(1.0));
      canvas.set_stroke_width(2.0);
      canvas.stroke(&rect);

      // FIXME needs a style system to fix.
      //   if !w.state.label.text().as_ref().is_empty() {
      //      w.state.label.on_draw(canvas);
      //   }
   }

   pub fn on_mouse_enter(w: &mut Self) {
      w.state.is_hover = true;
      w.base.request_draw();
   }

   pub fn on_mouse_leave(w: &mut Self) {
      w.state.is_hover = false;
      w.base.request_draw();
   }

   pub fn on_mouse_button(w: &mut Self, event: &MouseButtonsEventCtx) -> bool {
      match event.input.state {
         MouseState::Pressed => {
            if w.state.is_hover {
               w.state.is_down = true;
               w.handler.pressed(&w.state, event.input.button);
               w.base.request_draw();
               return true;
            }
         }
         MouseState::Released => {
            if w.state.is_down {
               w.state.is_down = false;
               w.handler.released(&w.state, event.input.button);

               if w.state.is_hover {
                  w.state.toggle += 1;
                  if w.state.toggle == w.state.toggle_num {
                     w.state.toggle = 0;
                  }

                  w.handler.click(&w.state);
               }
               w.base.request_draw();
               return true;
            }
         }
      }

      false
   }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
   use super::*;
   use crate::widgets::{handler::ButtonHandler, style::ButtonStyle};

   #[test]
   fn sizes() {
      dbg!(std::mem::size_of::<Button<ButtonHandler, ()>>());
      dbg!(std::mem::size_of::<ButtonStyle>());
   }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
