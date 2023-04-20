pub mod handler;
pub mod render;

use self::render::{ButtonRenderObject, ButtonRenderObjectData};
use super::handler::IButtonHandler;
use crate::core::events::{
   DrawEventCtx, KeyboardEventCtx, LayoutEventCtx, LifecycleEventCtx, LifecycleState,
   MouseButtonsEventCtx, MouseMoveEventCtx, MouseWheelEventCtx, UpdateEventCtx,
};
use crate::core::input::mouse::MouseState;
use crate::core::{IWidget, Painter, WidgetBase, WidgetVt};
use crate::elements::Icon;
use crate::theme::ButtonDefined;
use anyhow::bail;
use bitflags::bitflags;
use m::Rect;
use std::borrow::Cow;
use std::rc::Rc;
use std::{any::Any, mem::MaybeUninit};

////////////////////////////////////////////////////////////////////////////////////////////////////

bitflags! {
   #[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
   pub struct ButtonStateFlags: u8 {
      const STYLE_ERROR_PRINTED = 1<<0;
      const STYLE_CUSTOM = 1<<1;
      const IS_DOWN = 1<<5;
   }
}

#[derive(Default)]
pub struct ButtonState {
   pub style_name: &'static str,
   pub text: Option<Cow<'static, str>>,
   pub icon: Option<Icon>,
   pub flags: ButtonStateFlags,
   pub toggle_num: u8,
   pub toggle: u8,
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

   render_obj: Option<Rc<dyn ButtonRenderObject>>,
   state: ButtonState,
}

impl<H, D> Default for Button<H, D>
where
   H: Default + IButtonHandler + 'static,
   D: Default + Any,
{
   fn default() -> Self {
      Self::inherit(|_| (D::default()), |_| {}, H::default())
   }
}

impl<H> Button<H, ()>
where
   H: IButtonHandler + 'static,
{
   /// Construct new button.
   pub fn new(handler: H) -> Self {
      Self::inherit(|_| (), |_| {}, handler)
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
            on_lifecycle: Self::on_lifecycle,
            on_draw: Self::on_draw,
            on_mouse_cross: Self::on_mouse_cross,
            on_mouse_button: Self::on_mouse_button,
            ..WidgetVt::default()
         },
         handler,
         render_obj: None,
         state: ButtonState { style_name: ButtonDefined::Normal.into(), ..ButtonState::default() },
      };

      let inherited = vt_cb(&mut out.vtable);
      out.inherited.write(inherited);

      init_cb(&mut out);
      out
   }

   /// Set button text.
   #[inline]
   pub fn set_icon(&mut self, icon: Option<Icon>) {
      self.state.icon = icon
   }

   /// Set button text.
   #[inline]
   pub fn set_text<T>(&mut self, text: Option<T>)
   where
      T: Into<Cow<'static, str>>,
   {
      self.state.text = text.map(|v| v.into())
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

   /// Set a style by its name.
   ///
   /// It search the specified style in the current theme.
   pub fn set_style<N>(&mut self, name: N) -> anyhow::Result<()>
   where
      N: Into<&'static str>,
   {
      self.state.style_name = name.into();
      if let Some(runtime) = self.base.runtime() {
         if let Some(style) = runtime.theme().buttons.get(self.state.style_name) {
            self.render_obj = Some(style);
         } else {
            bail!("Style with name <{}> was not found.", self.state.style_name);
         }
      }

      Ok(())
   }

   #[inline]
   pub fn style(&self) -> &str {
      self.state.style_name.as_ref()
   }

   /// Set a user defined style.
   #[inline]
   pub fn set_render_obj(&mut self, ro: Rc<dyn ButtonRenderObject>) {
      self.render_obj = Some(ro);
      self.state.flags.set(ButtonStateFlags::STYLE_CUSTOM, true);
      self.state.flags.remove(ButtonStateFlags::STYLE_ERROR_PRINTED);
   }

   /// Get current style.
   #[inline]
   pub fn render_obj(&self) -> Option<Rc<dyn ButtonRenderObject>> {
      self.render_obj.clone()
   }
}

impl<H, D> Button<H, D>
where
   H: IButtonHandler + 'static,
   D: Any,
{
   fn on_lifecycle(w: &mut Self, event: &LifecycleEventCtx) {
      match event.state {
         LifecycleState::RuntimeSet => {
            if !w.state.flags.contains(ButtonStateFlags::STYLE_CUSTOM) {
               if let Some(style) =
                  w.base.runtime().unwrap().theme().buttons.get(w.state.style_name)
               {
                  w.render_obj = Some(style);
                  w.state.flags.remove(ButtonStateFlags::STYLE_ERROR_PRINTED);
               } else {
                  w.state.flags.set(ButtonStateFlags::STYLE_ERROR_PRINTED, true);
                  log::error!(
                     "a button style with the name <{}> was not found",
                     w.state.style_name
                  );
               }
            }
         }
         _ => {}
      }
   }

   fn on_draw(w: &mut Self, canvas: &mut Painter, _event: &DrawEventCtx) {
      if let Some(style) = &w.render_obj {
         if let Some(runtime) = w.base.runtime() {
            let data = ButtonRenderObjectData {
               text: w.state.text.as_ref().map(|v| v.as_ref()),
               icon: w.state.icon.as_ref(),
               bounds: w.base.geometry().rect().into(),
               is_hover: w.base.is_over(),
               is_active: w.state.flags.contains(ButtonStateFlags::IS_DOWN),
               has_menu: false,
               has_focus: w.base.has_focus(),
               toggle_num: w.state.toggle_num,
               toggle_curr: w.state.toggle,
            };
            if w.base.is_enabled() {
               style.draw_enabled(runtime.theme(), &data, canvas);
            } else {
               style.draw_disabled(runtime.theme(), &data, canvas);
            }
         } else if !w.state.flags.contains(ButtonStateFlags::STYLE_ERROR_PRINTED) {
            w.state.flags.set(ButtonStateFlags::STYLE_ERROR_PRINTED, true);
            log::error!("a runtime is not set");
         }
      } else if !w.state.flags.contains(ButtonStateFlags::STYLE_ERROR_PRINTED) {
         w.state.flags.set(ButtonStateFlags::STYLE_ERROR_PRINTED, true);
         log::error!("a style is not set");
      }
   }

   pub fn on_mouse_cross(w: &mut Self, _enter: bool) {
      w.base.request_draw();
   }

   pub fn on_mouse_button(w: &mut Self, event: &MouseButtonsEventCtx) -> bool {
      match event.input.state {
         MouseState::Pressed => {
            if w.base.is_over() {
               w.state.flags.set(ButtonStateFlags::IS_DOWN, true);
               w.handler.pressed(&w.state, event.input.button);
               w.base.request_draw();
               return true;
            }
         }
         MouseState::Released => {
            if w.state.flags.contains(ButtonStateFlags::IS_DOWN) {
               w.state.flags.set(ButtonStateFlags::IS_DOWN, false);
               w.handler.released(&w.state, event.input.button);

               if w.base.is_over() {
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
   fn on_draw(&mut self, canvas: &mut Painter, event: &DrawEventCtx) {
      (self.vtable.on_draw)(self, canvas, event);
   }

   #[cfg_attr(feature = "trace-widget",
   tracing::instrument(skip(self, event), fields(WidgetID = self.base().id().raw(), ret)))]
   fn on_update(&mut self, event: &UpdateEventCtx) {
      (self.vtable.on_update)(self, event);
   }

   #[cfg_attr(feature = "trace-widget",
   tracing::instrument(skip(self), fields(WidgetID = self.base().id().raw()), ret))]
   fn on_mouse_cross(&mut self, enter: bool) {
      (self.vtable.on_mouse_cross)(self, enter)
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

////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
   use super::*;
   use crate::widgets::{handler::ButtonHandler, render::ButtonRender};

   #[test]
   fn sizes() {
      dbg!(std::mem::size_of::<Button<ButtonHandler, ()>>());
      dbg!(std::mem::size_of::<ButtonRenderObjectData>());
      dbg!(std::mem::size_of::<ButtonRender>());
   }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
