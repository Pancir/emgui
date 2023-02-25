use crate::core::Geometry;
use crate::widgets::{IWidget, WidgetId, WidgetRef};
use sim_draw::Canvas;
use std::cell::{Cell, RefCell};
use std::ops::DerefMut;
use std::rc::Rc;

////////////////////////////////////////////////////////////////////////////////////////////////////

/// Common data for all widgets.
pub struct BaseState {
   /// Unique within application instance widget id.
   pub id: WidgetId,

   /// Element geometry.
   pub geometry: Geometry,

   /// Element's parent.
   pub parent: Option<WidgetRef>,

   /// Self element reference.
   pub self_ref: Option<WidgetRef>,

   /// `True` if mouse over element.
   pub is_hover: bool,

   /// `True` if the element is enabled then user can interact with it.
   pub is_enabled: bool,

   /// `True` if the element should not be drawn.
   pub is_visible: bool,

   /// `True` if it is focused.
   pub has_focus: bool,

   //---------------------------
   /// `True` if element want draw event.
   needs_draw: Cell<bool>,

   /// `True` if element want to be destroyed.
   needs_del: Cell<bool>,

   //---------------------------
   children: RefCell<Vec<Rc<RefCell<dyn IWidget>>>>,
}

impl Default for BaseState {
   fn default() -> Self {
      Self {
         id: WidgetId::new(),
         geometry: Geometry::default(),
         parent: None,
         self_ref: None,
         is_hover: false,
         is_enabled: false,
         is_visible: false,
         has_focus: false,

         //---------------------------
         needs_draw: Cell::new(false),
         needs_del: Cell::new(false),
         children: Default::default(),
      }
   }
}

impl BaseState {
   /// Check whether the element wants to be destroyed.
   #[inline]
   pub fn needs_delete(&self) -> bool {
      self.needs_del.get()
   }

   /// Check whether the element wants a draw event.
   #[inline]
   pub fn needs_draw(&self, reset: bool) -> bool {
      let out = self.needs_draw.get();
      if reset {
         self.needs_draw.set(false);
      }
      out
   }

   /// Request delete.
   #[inline]
   pub fn request_delete(&self) {
      self.needs_draw.set(true);
   }

   /// Request draw event.
   pub fn request_draw(&self) {
      if !self.needs_draw.get() {
         self.needs_draw.set(true);
         if let Some(p) = &self.parent {
            if let Some(o) = p.upgrade() {
               o.borrow().base_state().request_draw();
            }
         }
      }
   }
}

impl BaseState {
   fn take_children(&self) -> Vec<Rc<RefCell<dyn IWidget>>> {
      std::mem::take(self.children.borrow_mut().deref_mut())
   }

   fn set_children(&self, mut ch: Vec<Rc<RefCell<dyn IWidget>>>) {
      *self.children.borrow_mut().deref_mut() = std::mem::take(&mut ch);
   }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn on_draw(child: &Rc<RefCell<dyn IWidget>>, canvas: &mut Canvas, force: bool) {
   let mut children = match child.try_borrow_mut() {
      Ok(mut child) => {
         let is_visible = child.base_state().is_visible;
         if is_visible || force {
            child.on_draw(canvas);
            child.base_state().take_children()
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

      match child.try_borrow_mut() {
         Ok(child) => {
            child.base_state().set_children(children);
         }
         Err(e) => {
            panic!("{}", e)
         }
      };
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
