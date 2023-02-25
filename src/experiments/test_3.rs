use crate::core::Geometry;
use crate::widgets::events::{
   KeyboardEvent, LayoutEvent, LifecycleEvent, MouseButtonsEvent, MouseMoveEvent, MouseWheelEvent,
   UpdateEvent,
};
use crate::widgets::WidgetId;
use sim_draw::color::Rgba;
use sim_draw::{Canvas, Paint};
use std::any::Any;
use std::cell::RefCell;
use std::mem::MaybeUninit;
use std::ops::DerefMut;
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

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Default)]
pub struct Children {
   children: RefCell<Vec<Rc<RefCell<dyn IWidget>>>>,
}

impl Children {
   fn take(&self) -> Vec<Rc<RefCell<dyn IWidget>>> {
      std::mem::take(self.children.borrow_mut().deref_mut())
   }

   fn set(&self, mut ch: Vec<Rc<RefCell<dyn IWidget>>>) {
      *self.children.borrow_mut().deref_mut() = std::mem::take(&mut ch);
   }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

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

   fn request_draw(&mut self);

   fn request_delete(&mut self);

   fn request_update(&mut self);

   //---------------------------------------

   fn derive(&self) -> &dyn Derive;

   fn derive_mut(&mut self) -> &mut dyn Derive;

   fn children(&self) -> &Children;

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

   parent: Option<Weak<RefCell<dyn IWidget>>>,
   needs_update: bool,
   needs_draw: bool,
   needs_del: bool,
}

impl<D: 'static> Widget<D>
where
   D: Derive,
{
   pub fn new<CB>(cb: CB) -> Rc<RefCell<Self>>
   where
      CB: Fn(&mut WidgetVt<Self>, Weak<RefCell<Widget<D>>>) -> D,
   {
      let out = Self {
         id: WidgetId::new(),
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
         parent: None,
         needs_update: false,
         needs_draw: false,
         needs_del: false,
      };

      let out = Rc::new(RefCell::new(out));
      {
         let self_ref = Rc::downgrade(&out);
         let mut bor = out.borrow_mut();
         let derive = cb(&mut bor.vtable, self_ref);
         bor.derive.write(derive);
      }

      out
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

   fn request_draw(&mut self) {
      if !self.needs_draw {
         self.needs_draw = true;
         if let Some(p) = &self.parent {
            if let Some(o) = p.upgrade() {
               o.borrow_mut().request_draw();
            }
         }
      }
   }

   fn request_delete(&mut self) {
      self.needs_del = true;
   }

   fn request_update(&mut self) {
      if !self.needs_update {
         self.needs_update = true;
         if let Some(p) = &self.parent {
            if let Some(o) = p.upgrade() {
               o.borrow_mut().request_update();
            }
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

   fn children(&self) -> &Children {
      &self.children
   }

   fn is_visible(&self) -> bool {
      true
   }

   //---------------------------------------

   fn is_enabled(&self) -> bool {
      true
   }

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
      canvas.set_paint(Paint::new_color(Rgba::GRAY));
      canvas.fill(&self.geometry.rect());
   }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn emit_lifecycle(child: &Rc<RefCell<dyn IWidget>>, event: &LifecycleEvent) {
   let mut children = match child.try_borrow_mut() {
      Ok(mut child) => {
         child.emit_lifecycle(event);
         child.children().take()
      }
      Err(e) => {
         panic!("{}", e)
      }
   };

   if !children.is_empty() {
      emit_lifecycle_children(&mut children, event);
      child.try_borrow_mut().unwrap_or_else(|e| panic!("{}", e)).children().set(children);
   }
}

fn emit_lifecycle_children(children: &mut Vec<Rc<RefCell<dyn IWidget>>>, event: &LifecycleEvent) {
   for child in children {
      emit_lifecycle(&child, event);
   }
}

//------------------------------------------------------------------------------------------------//

pub fn emit_layout(child: &Rc<RefCell<dyn IWidget>>, event: &LayoutEvent) {
   let mut children = match child.try_borrow_mut() {
      Ok(mut child) => {
         child.emit_layout(event);
         child.children().take()
      }
      Err(e) => {
         panic!("{}", e)
      }
   };

   if !children.is_empty() {
      emit_layout_children(&mut children, event);
      child.try_borrow_mut().unwrap_or_else(|e| panic!("{}", e)).children().set(children);
   }
}

fn emit_layout_children(children: &mut Vec<Rc<RefCell<dyn IWidget>>>, event: &LayoutEvent) {
   for child in children {
      emit_layout(&child, event);
   }
}

//------------------------------------------------------------------------------------------------//

pub fn emit_update(child: &Rc<RefCell<dyn IWidget>>, event: &UpdateEvent) {
   let mut children = match child.try_borrow_mut() {
      Ok(mut child) => {
         child.emit_update(event);
         child.children().take()
      }
      Err(e) => {
         panic!("{}", e)
      }
   };

   if !children.is_empty() {
      emit_update_children(&mut children, event);
      child.try_borrow_mut().unwrap_or_else(|e| panic!("{}", e)).children().set(children);
   }
}

fn emit_update_children(children: &mut Vec<Rc<RefCell<dyn IWidget>>>, event: &UpdateEvent) {
   for child in children {
      emit_update(&child, event);
   }
}

//------------------------------------------------------------------------------------------------//

pub fn emit_draw(child: &Rc<RefCell<dyn IWidget>>, canvas: &mut Canvas, force: bool) {
   let mut children = match child.try_borrow_mut() {
      Ok(mut child) => {
         let is_visible = child.is_visible();
         if is_visible || force {
            child.emit_draw(canvas);
            child.children().take()
         } else {
            Vec::default()
         }
      }
      Err(e) => {
         panic!("{}", e)
      }
   };

   if !children.is_empty() {
      emit_draw_children(&mut children, canvas, force);
      child.try_borrow_mut().unwrap_or_else(|e| panic!("{}", e)).children().set(children);
   }
}

fn emit_draw_children(
   children: &mut Vec<Rc<RefCell<dyn IWidget>>>,
   canvas: &mut Canvas,
   force: bool,
) {
   for child in children {
      emit_draw(&child, canvas, force);
   }
}

//------------------------------------------------------------------------------------------------//

pub fn emit_mouse_move(child: &Rc<RefCell<dyn IWidget>>, event: &MouseMoveEvent) -> bool {
   let mut children = match child.try_borrow_mut() {
      Ok(child) => child.children().take(),
      Err(e) => {
         panic!("{}", e)
      }
   };

   if !children.is_empty() {
      emit_mouse_move_children(&mut children, event);
   }

   match child.try_borrow_mut() {
      Ok(mut child) => {
         child.children().set(children);
         if child.emit_mouse_move(event) {
            return true;
         }
      }
      Err(e) => {
         panic!("{}", e)
      }
   };

   false
}

fn emit_mouse_move_children(
   children: &mut Vec<Rc<RefCell<dyn IWidget>>>,
   event: &MouseMoveEvent,
) -> bool {
   for child in children {
      if emit_mouse_move(&child, event) {
         return true;
      }
   }
   false
}

//------------------------------------------------------------------------------------------------//

pub fn emit_mouse_button(child: &Rc<RefCell<dyn IWidget>>, event: &MouseButtonsEvent) -> bool {
   let mut children = match child.try_borrow_mut() {
      Ok(child) => child.children().take(),
      Err(e) => {
         panic!("{}", e)
      }
   };

   if !children.is_empty() {
      emit_mouse_button_children(&mut children, event);
   }

   match child.try_borrow_mut() {
      Ok(mut child) => {
         child.children().set(children);
         if child.emit_mouse_button(event) {
            return true;
         }
      }
      Err(e) => {
         panic!("{}", e)
      }
   };

   false
}

fn emit_mouse_button_children(
   children: &mut Vec<Rc<RefCell<dyn IWidget>>>,
   event: &MouseButtonsEvent,
) -> bool {
   for child in children {
      if emit_mouse_button(&child, event) {
         return true;
      }
   }
   false
}

//------------------------------------------------------------------------------------------------//

pub fn emit_mouse_wheel(child: &Rc<RefCell<dyn IWidget>>, event: &MouseWheelEvent) -> bool {
   let mut children = match child.try_borrow_mut() {
      Ok(child) => child.children().take(),
      Err(e) => {
         panic!("{}", e)
      }
   };

   if !children.is_empty() {
      emit_mouse_wheel_children(&mut children, event);
   }

   match child.try_borrow_mut() {
      Ok(mut child) => {
         child.children().set(children);
         if child.emit_mouse_wheel(event) {
            return true;
         }
      }
      Err(e) => {
         panic!("{}", e)
      }
   };

   false
}

fn emit_mouse_wheel_children(
   children: &mut Vec<Rc<RefCell<dyn IWidget>>>,
   event: &MouseWheelEvent,
) -> bool {
   for child in children {
      if emit_mouse_wheel(&child, event) {
         return true;
      }
   }
   false
}

//------------------------------------------------------------------------------------------------//

pub fn emit_keyboard(child: &Rc<RefCell<dyn IWidget>>, event: &KeyboardEvent) -> bool {
   let mut children = match child.try_borrow_mut() {
      Ok(child) => child.children().take(),
      Err(e) => {
         panic!("{}", e)
      }
   };

   if !children.is_empty() {
      emit_mouse_keyboard_children(&mut children, event);
   }

   match child.try_borrow_mut() {
      Ok(mut child) => {
         child.children().set(children);
         if child.emit_keyboard(event) {
            return true;
         }
      }
      Err(e) => {
         panic!("{}", e)
      }
   };

   false
}

fn emit_mouse_keyboard_children(
   children: &mut Vec<Rc<RefCell<dyn IWidget>>>,
   event: &KeyboardEvent,
) -> bool {
   for child in children {
      if emit_keyboard(&child, event) {
         return true;
      }
   }
   false
}

////////////////////////////////////////////////////////////////////////////////////////////////////
