use sim_draw::Canvas;
use std::any::Any;
use std::cell::RefCell;
use std::ops::DerefMut;
use std::rc::{Rc, Weak};

////////////////////////////////////////////////////////////////////////////////////////////////////

pub trait Derive: Any + 'static {
   fn as_any(&self) -> &dyn Any;
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

pub trait IWidget: Any + 'static {
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

   //---------------------------------------

   fn children(&self) -> &Children;

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
   derive: D,
   vtable: WidgetVt<Self>,
   children: Children,
}

impl<D: 'static> Widget<D>
where
   D: Derive,
{
   pub fn new(derive: D, cb: impl Fn(&mut WidgetVt<Self>)) -> Rc<RefCell<Self>> {
      let mut out = Self {
         derive,
         vtable: WidgetVt { on_draw: Self::on_draw },
         children: Children::default(),
      };

      cb(&mut out.vtable);
      let out = Rc::new(RefCell::new(out));
      let self_ref = Rc::downgrade(&out);

      out.borrow_mut().derive.set_ref(self_ref);

      out
   }
}

impl<D: 'static> IWidget for Widget<D>
where
   D: Derive,
{
   fn children(&self) -> &Children {
      &self.children
   }

   fn is_visible(&self) -> bool {
      true
   }

   fn on_draw(&mut self, _canvas: &mut Canvas) {}
}

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
