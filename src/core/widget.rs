use crate::core::control::Internal;
use crate::core::derive::Derive;
use crate::core::events::{
   DrawEventCtx, KeyboardEventCtx, LayoutEventCtx, LifecycleEventCtx, LifecycleState,
   MouseButtonsEventCtx, MouseMoveEventCtx, MouseWheelEventCtx, UpdateEventCtx,
};
use crate::core::{Geometry, WidgetId};
use crate::defines::{DEFAULT_DOUBLE_CLICK_TIME, DEFAULT_TOOL_TIP_TIME};
use sim_draw::color::Rgba;
use sim_draw::m::Rect;
use sim_draw::{Canvas, Paint};
use std::any::Any;
use std::cell::RefCell;
use std::mem::MaybeUninit;
use std::rc::{Rc, Weak};
use std::time::Duration;

////////////////////////////////////////////////////////////////////////////////////////////////////

/// Cast dyn [IWidget] to specified type.
pub fn cast<T: Derive>(_input: Weak<RefCell<&dyn IWidget>>) -> Weak<RefCell<Widget<T>>> {
   unimplemented!()
}

pub trait IWidget: Any + 'static {
   fn as_any(&self) -> &dyn Any;
   fn as_any_mut(&mut self) -> &mut dyn Any;

   /// Get the widget type name for debugging purposes.
   /// Developers should not override this method.
   fn type_name(&self) -> &'static str {
      std::any::type_name::<Self>()
   }

   /// Get the widget type name for debugging purposes.
   ///
   /// Developers should not override this method.
   fn type_name_short(&self) -> &'static str {
      let name = self.type_name();
      name.split('<').next().unwrap_or(name).split("::").last().unwrap_or(name)
   }

   fn id(&self) -> WidgetId;

   //---------------------------------------

   fn request_draw(&self);
   fn request_delete(&self);
   fn request_update(&self);
   fn request_focus(&self);

   //---------------------------------------

   fn internal(&self) -> &Internal;
   fn internal_mut(&mut self) -> &mut Internal;

   //---------------------------------------

   fn set_tool_type_time(&mut self, duration: Option<Duration>);
   fn tool_type_time(&self) -> Duration;

   fn set_double_click_time(&mut self, duration: Option<Duration>);
   fn double_click_time(&self) -> Duration;

   //---------------------------------------

   fn derive(&self) -> &dyn Derive;
   fn derive_mut(&mut self) -> &mut dyn Derive;

   fn geometry(&self) -> &Geometry;

   fn set_rect(&mut self, r: Rect<f32>);
   fn set_background_color(&mut self, color: Rgba);

   //---------------------------------------

   fn is_visible(&self) -> bool;
   fn set_visible(&mut self, state: bool);

   fn is_enabled(&self) -> bool;
   fn set_enabled(&mut self, state: bool);

   fn is_transparent(&self) -> bool;
   fn set_transparent(&mut self, state: bool);

   //---------------------------------------

   ///This property holds whether mouse tracking is enabled for the widget.
   /// If mouse tracking is disabled (the default), the widget only receives
   /// mouse move events when at least one mouse button is pressed while the mouse is being moved.
   /// If mouse tracking is enabled, the widget receives mouse move events even if no buttons are pressed.
   ///
   /// See [Self::has_mouse_tracking]
   fn set_mouse_tracking(&mut self, state: bool);

   /// See [Self::set_mouse_tracking]
   fn has_mouse_tracking(&mut self) -> bool;

   /// `True` if mouse is over the widget.
   fn is_mouse_over(&self) -> bool;

   //---------------------------------------

   fn emit_lifecycle(&mut self, _event: &LifecycleEventCtx);
   fn emit_layout(&mut self, _event: &LayoutEventCtx);
   fn emit_draw(&mut self, _canvas: &mut Canvas, event: &DrawEventCtx);
   fn emit_update(&mut self, _event: &UpdateEventCtx);
   fn emit_mouse_enter(&mut self);
   fn emit_mouse_leave(&mut self);
   fn emit_mouse_move(&mut self, _event: &MouseMoveEventCtx) -> bool;
   fn emit_mouse_button(&mut self, _event: &MouseButtonsEventCtx) -> bool;
   fn emit_mouse_wheel(&mut self, _event: &MouseWheelEventCtx) -> bool;
   fn emit_keyboard(&mut self, _event: &KeyboardEventCtx) -> bool;
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Copy, Clone)]
pub struct WidgetVt<D> {
   /// It is called when a new rectangle has to be set.
   pub on_set_rect: fn(w: &mut D, Rect<f32>) -> Option<Rect<f32>>,
   //-------------------------------------------------
   pub on_visible: fn(w: &mut D, bool),
   pub on_disable: fn(w: &mut D, bool),
   //-------------------------------------------------
   pub on_lifecycle: fn(w: &mut D, &LifecycleEventCtx),
   pub on_layout: fn(w: &mut D, &LayoutEventCtx),
   //-------------------------------------------------
   pub on_update: fn(w: &mut D, &UpdateEventCtx),
   pub on_draw: fn(w: &mut D, &mut Canvas, &DrawEventCtx),
   //-------------------------------------------------
   /// An event is sent to the widget when the mouse cursor enters the widget.
   pub on_mouse_enter: fn(w: &mut D),

   /// A leave event is sent to the widget when the mouse cursor leaves the widget.
   pub on_mouse_leave: fn(w: &mut D),
   //-------------------------------------------------
   /// If mouse tracking is switched off, mouse move events only occur if
   /// a mouse button is pressed while the mouse is being moved.
   /// If [mouse tracking](IWidget::set_mouse_tracking) is switched on,
   /// mouse move events occur even if **NO** mouse button is pressed.
   pub on_mouse_move: fn(w: &mut D, &MouseMoveEventCtx) -> bool,
   pub on_mouse_button: fn(w: &mut D, &MouseButtonsEventCtx) -> bool,
   pub on_mouse_wheel: fn(w: &mut D, &MouseWheelEventCtx) -> bool,

   pub on_keyboard: fn(w: &mut D, &KeyboardEventCtx) -> bool,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct Widget<D>
where
   D: Derive,
{
   derive: MaybeUninit<D>,
   vtable: WidgetVt<Self>,
   internal: Internal,
}

impl<D: 'static> Widget<D>
where
   D: Derive,
{
   /// Construct new and init.
   pub fn new_flat<CB1, CB2>(vt_cb: CB1, init_cb: CB2) -> Self
   where
      CB1: FnOnce(&mut WidgetVt<Self>) -> D,
      CB2: FnOnce(&mut Self),
   {
      let mut out = Self {
         derive: unsafe { std::mem::zeroed() },
         vtable: WidgetVt {
            on_set_rect: |_, v| Some(v),
            //--------------------------------------
            on_visible: |_, _| {},
            on_disable: |_, _| {},
            //--------------------------------------
            on_lifecycle: |_, _| {},
            on_layout: |_, _| {},
            //--------------------------------------
            on_update: |_, _| {},
            on_draw: Self::on_draw,
            //--------------------------------------
            // TODO remove draw as it is for testing
            on_mouse_enter: |w| w.request_draw(),
            on_mouse_leave: |w| w.request_draw(),
            //--------------------------------------
            on_mouse_move: |_, _| true,
            on_mouse_button: |_, _| true,
            on_mouse_wheel: |_, _| true,
            //--------------------------------------
            on_keyboard: |_, _| false,
         },
         internal: Internal::new::<Self>(),
      };

      let derive = vt_cb(&mut out.vtable);
      out.derive.write(derive);

      init_cb(&mut out);
      out
   }

   /// Construct new `Rc` and init.
   pub fn new<CB1, CB2>(vt_cb: CB1, init_cb: CB2) -> Rc<RefCell<Self>>
   where
      CB1: FnOnce(&mut WidgetVt<Self>) -> D,
      CB2: FnOnce(&mut Self),
   {
      Self::new_flat(vt_cb, init_cb).to_rc()
   }

   /// Plane into `Rc`.
   ///
   /// TODO maybe Pin?
   pub fn to_rc(self) -> Rc<RefCell<Self>> {
      let w = Rc::new(RefCell::new(self));
      let d = Rc::downgrade(&w);
      match w.try_borrow_mut() {
         Ok(mut w) => (w.vtable.on_lifecycle)(
            &mut w,
            &LifecycleEventCtx { state: LifecycleState::SelfReference(d) },
         ),
         Err(_) => {
            unreachable!()
         }
      }
      w
   }

   #[inline]
   pub fn derive_ref(&self) -> &D {
      // # Safety
      // All initialization happen in new function.
      unsafe { self.derive.assume_init_ref() }
   }

   #[inline]
   pub fn derive_mut(&mut self) -> &mut D {
      // # Safety
      // All initialization happen in new function.
      unsafe { self.derive.assume_init_mut() }
   }
}

impl<D: 'static> IWidget for Widget<D>
where
   D: Derive,
{
   fn as_any(&self) -> &dyn Any {
      self
   }

   fn as_any_mut(&mut self) -> &mut dyn Any {
      self
   }

   fn id(&self) -> WidgetId {
      self.internal.id
   }

   //---------------------------------------

   #[cfg_attr(feature = "trace-widget",
   tracing::instrument(skip(self), fields(WidgetID = self.id().raw())))]
   fn request_draw(&self) {
      self.internal.request_draw();
   }

   #[cfg_attr(feature = "trace-widget",
   tracing::instrument(skip(self), fields(WidgetID = self.id().raw())))]
   fn request_delete(&self) {
      self.internal.request_delete();
   }

   #[cfg_attr(feature = "trace-widget",
   tracing::instrument(skip(self), fields(WidgetID = self.id().raw())))]
   fn request_update(&self) {
      self.internal.request_update();
   }

   #[cfg_attr(feature = "trace-widget",
   tracing::instrument(skip(self), fields(WidgetID = self.id().raw())))]
   fn request_focus(&self) {
      unimplemented!()
   }

   //---------------------------------------

   fn internal(&self) -> &Internal {
      &self.internal
   }

   fn internal_mut(&mut self) -> &mut Internal {
      &mut self.internal
   }

   //---------------------------------------

   fn set_tool_type_time(&mut self, duration: Option<Duration>) {
      self.internal.tool_type_time = duration;
   }

   fn tool_type_time(&self) -> Duration {
      if let Some(v) = self.internal.tool_type_time {
         v
      } else {
         if let Some(r) = &self.internal.runtime {
            r.tool_type_time()
         } else {
            DEFAULT_TOOL_TIP_TIME
         }
      }
   }

   fn set_double_click_time(&mut self, duration: Option<Duration>) {
      self.internal.double_click_time = duration;
   }

   fn double_click_time(&self) -> Duration {
      if let Some(v) = self.internal.double_click_time {
         v
      } else {
         if let Some(r) = &self.internal.runtime {
            r.double_click_time()
         } else {
            DEFAULT_DOUBLE_CLICK_TIME
         }
      }
   }

   //---------------------------------------

   fn derive(&self) -> &dyn Derive {
      self.derive_ref()
   }

   fn derive_mut(&mut self) -> &mut dyn Derive {
      self.derive_mut()
   }

   fn geometry(&self) -> &Geometry {
      &self.internal.geometry
   }

   fn set_rect(&mut self, r: Rect<f32>) {
      if let Some(rect) = (self.vtable.on_set_rect)(self, r) {
         self.internal.geometry.set_rect(rect);
      }
   }

   fn set_background_color(&mut self, color: Rgba) {
      debug_assert!(color.a > (0.0 - f32::EPSILON) && color.a < 1.0 + f32::EPSILON);
      self.internal.background_color = color;
      self.set_transparent(color.a < 1.0);
   }

   //---------------------------------------

   fn is_visible(&self) -> bool {
      self.internal.is_visible()
   }

   fn set_visible(&mut self, state: bool) {
      if self.internal.set_visible(state) {
         (self.vtable.on_visible)(self, state)
      }
   }

   fn is_enabled(&self) -> bool {
      self.internal.is_enabled()
   }

   fn set_enabled(&mut self, state: bool) {
      if self.internal.set_enabled(state) {
         (self.vtable.on_disable)(self, !state)
      }
   }

   fn is_transparent(&self) -> bool {
      self.internal.is_transparent()
   }

   fn set_transparent(&mut self, state: bool) {
      self.internal.set_transparent(state)
   }

   //---------------------------------------

   fn set_mouse_tracking(&mut self, state: bool) {
      self.internal.set_mouse_tracking(state)
   }

   fn has_mouse_tracking(&mut self) -> bool {
      self.internal.has_mouse_tracking()
   }

   fn is_mouse_over(&self) -> bool {
      self.internal.is_over()
   }

   //---------------------------------------

   #[cfg_attr(feature = "trace-widget",
   tracing::instrument(skip(self, event), fields(WidgetID = self.id().raw(), ret)))]
   fn emit_lifecycle(&mut self, event: &LifecycleEventCtx) {
      (self.vtable.on_lifecycle)(self, event);
   }

   #[cfg_attr(feature = "trace-widget",
   tracing::instrument(skip(self, event), fields(WidgetID = self.id().raw(), ret)))]
   fn emit_layout(&mut self, event: &LayoutEventCtx) {
      (self.vtable.on_layout)(self, event);
   }

   #[cfg_attr(feature = "trace-widget",
   tracing::instrument(skip(self, canvas, event), fields(WidgetID = self.id().raw(), ret)))]
   fn emit_draw(&mut self, canvas: &mut Canvas, event: &DrawEventCtx) {
      (self.vtable.on_draw)(self, canvas, event);
   }

   #[cfg_attr(feature = "trace-widget",
   tracing::instrument(skip(self, event), fields(WidgetID = self.id().raw(), ret)))]
   fn emit_update(&mut self, event: &UpdateEventCtx) {
      (self.vtable.on_update)(self, event);
   }

   #[must_use]
   #[cfg_attr(feature = "trace-widget",
   tracing::instrument(skip(self), fields(WidgetID = self.id().raw()), ret))]
   fn emit_mouse_enter(&mut self) {
      (self.vtable.on_mouse_enter)(self)
   }

   #[must_use]
   #[cfg_attr(feature = "trace-widget",
   tracing::instrument(skip(self), fields(WidgetID = self.id().raw()), ret))]
   fn emit_mouse_leave(&mut self) {
      (self.vtable.on_mouse_leave)(self)
   }

   #[must_use]
   #[cfg_attr(feature = "trace-widget",
   tracing::instrument(skip(self, event), fields(WidgetID = self.id().raw()), ret))]
   fn emit_mouse_move(&mut self, event: &MouseMoveEventCtx) -> bool {
      (self.vtable.on_mouse_move)(self, event)
   }

   #[must_use]
   #[cfg_attr(feature = "trace-widget",
   tracing::instrument(skip(self, event), fields(WidgetID = self.id().raw()), ret))]
   fn emit_mouse_button(&mut self, event: &MouseButtonsEventCtx) -> bool {
      (self.vtable.on_mouse_button)(self, event)
   }

   #[must_use]
   #[cfg_attr(feature = "trace-widget",
   tracing::instrument(skip(self, event), fields(WidgetID = self.id().raw()), ret))]
   fn emit_mouse_wheel(&mut self, event: &MouseWheelEventCtx) -> bool {
      (self.vtable.on_mouse_wheel)(self, event)
   }

   #[must_use]
   #[cfg_attr(feature = "trace-widget",
   tracing::instrument(skip(self, event), fields(WidgetID = self.id().raw()), ret))]
   fn emit_keyboard(&mut self, event: &KeyboardEventCtx) -> bool {
      (self.vtable.on_keyboard)(self, event)
   }
}

impl<D: 'static> Widget<D>
where
   D: Derive,
{
   fn on_draw(&mut self, canvas: &mut Canvas, _event: &DrawEventCtx) {
      // TODO remove over as it is for testing
      if self.internal.is_over() {
         canvas.set_paint(Paint::new_color(Rgba::RED.with_alpha_mul(0.2)));
      } else {
         canvas.set_paint(Paint::new_color(self.internal.background_color));
      }
      canvas.fill(&self.internal.geometry.rect());
   }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
   use super::*;

   #[test]
   fn sizes() {
      dbg!(std::mem::size_of::<WidgetVt<()>>());
      dbg!(std::mem::size_of::<Widget<()>>());
   }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
