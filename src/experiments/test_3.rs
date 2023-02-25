use crate::widgets::WidgetId;
use sim_draw::Canvas;
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

   fn derive(&self) -> &dyn Derive;

   fn derive_mut(&mut self) -> &mut dyn Derive;

   fn children(&self) -> &Children;

   fn request_draw(&mut self);

   fn request_delete(&mut self);

   fn is_visible(&self) -> bool;

   //---------------------------------------

   fn on_draw(&mut self, _canvas: &mut Canvas);
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Copy, Clone)]
pub struct WidgetVt<D> {
   pub on_draw: fn(w: &mut D, &mut Canvas),
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

   parent: Option<Weak<RefCell<dyn IWidget>>>,
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
         vtable: WidgetVt { on_draw: Self::on_draw },
         children: Children::default(),
         //-----------
         parent: None,
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

   fn derive(&self) -> &dyn Derive {
      self.derive_ref()
   }

   fn derive_mut(&mut self) -> &mut dyn Derive {
      self.derive_mut()
   }

   fn children(&self) -> &Children {
      &self.children
   }

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

   fn is_visible(&self) -> bool {
      true
   }

   fn on_draw(&mut self, canvas: &mut Canvas) {
      (self.vtable.on_draw)(self, canvas)
   }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn on_draw(child: &Rc<RefCell<dyn IWidget>>, canvas: &mut Canvas, force: bool) {
   let mut children = match child.try_borrow_mut() {
      Ok(mut child) => {
         let is_visible = child.is_visible();
         if is_visible || force {
            child.on_draw(canvas);
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
      on_draw_children(&mut children, canvas, force);
      child.try_borrow_mut().unwrap_or_else(|e| panic!("{}", e)).children().set(children);
   }
}

fn on_draw_children(
   children: &mut Vec<Rc<RefCell<dyn IWidget>>>,
   canvas: &mut Canvas,
   force: bool,
) {
   for child in children {
      on_draw(&child, canvas, force);
   }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
