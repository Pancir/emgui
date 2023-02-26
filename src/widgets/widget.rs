use crate::core::Geometry;
use crate::widgets::children::Children;
use crate::widgets::events::{
   KeyboardEvent, LayoutEvent, LifecycleEvent, MouseButtonsEvent, MouseMoveEvent, MouseWheelEvent,
   UpdateEvent,
};
use crate::widgets::WidgetId;
use sim_draw::color::Rgba;
use sim_draw::m::Rect;
use sim_draw::{Canvas, Paint};
use std::any::Any;
use std::cell::{Cell, RefCell};
use std::mem::MaybeUninit;
use std::rc::{Rc, Weak};

////////////////////////////////////////////////////////////////////////////////////////////////////

pub trait Derive: Any + 'static {
   fn as_any(&self) -> &dyn Any;
   fn as_any_mut(&mut self) -> &mut dyn Any;

   fn derive_void(&self) -> Option<&dyn Derive> {
      None
   }
}

pub fn derive<T: 'static>(derive: &dyn Derive) -> Option<&T> {
   let mut value = derive.derive_void();
   loop {
      if let Some(d) = value {
         let any = d.as_any();
         if let Some(res) = any.downcast_ref::<T>() {
            return Some(res);
         }
         value = d.derive_void();
      } else {
         break;
      }
   }
   None
}

impl Derive for () {
   fn as_any(&self) -> &dyn Any {
      self
   }

   fn as_any_mut(&mut self) -> &mut dyn Any {
      self
   }
}

/// Cast dyn [IWidget] to specified type.
pub fn cast<T: Derive>(_input: Weak<RefCell<&dyn IWidget>>) -> Weak<RefCell<Widget<T>>> {
   unimplemented!()
}

////////////////////////////////////////////////////////////////////////////////////////////////////

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

   //---------------------------------------

   fn children(&self) -> &Children;
   fn children_mut(&mut self) -> &mut Children;

   //---------------------------------------

   fn derive(&self) -> &dyn Derive;

   fn derive_mut(&mut self) -> &mut dyn Derive;

   fn geometry(&self) -> &Geometry;

   fn set_geometry(&mut self, g: Geometry);

   fn set_rect(&mut self, r: Rect<f32>);

   //---------------------------------------

   fn is_visible(&self) -> bool;

   fn is_enabled(&self) -> bool;

   //---------------------------------------

   fn emit_lifecycle(&mut self, _event: &LifecycleEvent);
   fn emit_layout(&mut self, _event: &LayoutEvent);
   fn emit_draw(&mut self, _canvas: &mut Canvas);
   fn emit_update(&mut self, _event: &UpdateEvent);
   fn emit_mouse_move(&mut self, _event: &MouseMoveEvent) -> bool;
   fn emit_mouse_button(&mut self, _event: &MouseButtonsEvent) -> bool;
   fn emit_mouse_wheel(&mut self, _event: &MouseWheelEvent) -> bool;
   fn emit_keyboard(&mut self, _event: &KeyboardEvent) -> bool;
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Copy, Clone)]
pub struct WidgetVt<D> {
   pub on_lifecycle: fn(w: &mut D, &LifecycleEvent),
   pub on_layout: fn(w: &mut D, &LayoutEvent),
   pub on_update: fn(w: &mut D, &UpdateEvent),
   pub on_draw: fn(w: &mut D, &mut Canvas),
   pub on_mouse_move: fn(w: &mut D, &MouseMoveEvent) -> bool,
   pub on_mouse_button: fn(w: &mut D, &MouseButtonsEvent) -> bool,
   pub on_mouse_wheel: fn(w: &mut D, &MouseWheelEvent) -> bool,
   pub on_keyboard: fn(w: &mut D, &KeyboardEvent) -> bool,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct Widget<D>
where
   D: Derive,
{
   id: WidgetId,
   derive: MaybeUninit<D>,
   vtable: WidgetVt<Self>,
   children: Children,

   geometry: Geometry,

   needs_update: Cell<bool>,
   needs_draw: Cell<bool>,
   needs_del: Cell<bool>,
}

impl<D: 'static> Widget<D>
where
   D: Derive,
{
   /// Construct new.
   pub fn new<CB>(cb: CB) -> Rc<RefCell<Self>>
   where
      CB: FnOnce(&mut WidgetVt<Self>) -> D,
   {
      let mut out = Self {
         id: WidgetId::new::<Self>(),
         derive: unsafe { std::mem::zeroed() },
         vtable: WidgetVt {
            on_lifecycle: |_, _| {},
            on_layout: |_, _| {},
            on_update: |_, _| {},
            on_draw: Self::on_draw,
            on_mouse_move: |_, _| false,
            on_mouse_button: |_, _| false,
            on_mouse_wheel: |_, _| false,
            on_keyboard: |_, _| false,
         },
         children: Children::default(),
         geometry: Geometry::default(),
         //-----------
         needs_update: Cell::new(false),
         needs_draw: Cell::new(false),
         needs_del: Cell::new(false),
      };

      let derive = cb(&mut out.vtable);
      out.derive.write(derive);
      Rc::new(RefCell::new(out))
   }

   pub fn derive_ref(&self) -> &D {
      // # Safety
      // All initialization happen in new function.
      unsafe { self.derive.assume_init_ref() }
   }

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
      self.id
   }

   //---------------------------------------

   fn request_draw(&self) {
      if !self.needs_draw.get() {
         self.needs_draw.set(true);
         self.children.request_draw_parent();
      }
   }

   fn request_delete(&self) {
      if !self.needs_update.get() {
         self.needs_update.set(true);
         self.children.request_update_parent();
      }
   }

   fn request_update(&self) {
      self.needs_del.set(true);
   }
   //---------------------------------------

   fn children(&self) -> &Children {
      &self.children
   }

   fn children_mut(&mut self) -> &mut Children {
      &mut self.children
   }

   //---------------------------------------

   fn derive(&self) -> &dyn Derive {
      self.derive_ref()
   }

   fn derive_mut(&mut self) -> &mut dyn Derive {
      self.derive_mut()
   }

   fn geometry(&self) -> &Geometry {
      &self.geometry
   }

   fn set_geometry(&mut self, g: Geometry) {
      self.geometry = g;
   }

   fn set_rect(&mut self, r: Rect<f32>) {
      self.geometry.set_rect(r);
   }

   //---------------------------------------

   fn is_visible(&self) -> bool {
      true
   }

   fn is_enabled(&self) -> bool {
      true
   }

   //---------------------------------------

   fn emit_lifecycle(&mut self, event: &LifecycleEvent) {
      (self.vtable.on_lifecycle)(self, event);
   }

   fn emit_layout(&mut self, event: &LayoutEvent) {
      (self.vtable.on_layout)(self, event);
   }

   fn emit_draw(&mut self, canvas: &mut Canvas) {
      (self.vtable.on_draw)(self, canvas);
   }

   fn emit_update(&mut self, event: &UpdateEvent) {
      (self.vtable.on_update)(self, event);
   }

   #[must_use]
   fn emit_mouse_move(&mut self, event: &MouseMoveEvent) -> bool {
      (self.vtable.on_mouse_move)(self, event)
   }

   #[must_use]
   fn emit_mouse_button(&mut self, event: &MouseButtonsEvent) -> bool {
      (self.vtable.on_mouse_button)(self, event)
   }

   #[must_use]
   fn emit_mouse_wheel(&mut self, event: &MouseWheelEvent) -> bool {
      (self.vtable.on_mouse_wheel)(self, event)
   }

   #[must_use]
   fn emit_keyboard(&mut self, event: &KeyboardEvent) -> bool {
      (self.vtable.on_keyboard)(self, event)
   }
}

impl<D: 'static> Widget<D>
where
   D: Derive,
{
   fn on_draw(&mut self, canvas: &mut Canvas) {
      canvas.set_paint(Paint::new_color(Rgba::RED));
      canvas.fill(&self.geometry.rect());
   }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
   use super::*;

   #[test]
   fn sizes() {
      println!("{} : {}", std::any::type_name::<Widget<()>>(), std::mem::size_of::<Widget<()>>());
   }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
