use crate::core::Geometry;
use crate::widgets::{IWidget, WidgetId};
use std::cell::Cell;
use std::rc::Weak;

////////////////////////////////////////////////////////////////////////////////////////////////////

/// Common data for all widgets.
pub struct BaseState {
   /// Unique within application instance widget id.
   pub id: WidgetId,

   /// Element geometry.
   pub geometry: Geometry,

   /// Element's parent.
   pub parent: Option<Weak<dyn IWidget>>,

   /// Self element reference.
   pub self_ref: Option<Weak<dyn IWidget>>,

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
      }
   }
}

impl BaseState {
   /// Check whether the element wants to be destroyed.
   pub fn needs_delete(&self) -> bool {
      self.needs_del.get()
   }

   /// Check whether the element wants a draw event.
   pub fn needs_draw(&self) -> bool {
      self.needs_draw.get()
   }

   /// Request delete.
   pub fn request_delete(&self) {
      self.needs_draw.set(true);
   }

   /// Request draw event.
   pub fn request_draw(&self) {
      if !self.needs_draw.get() {
         self.needs_draw.set(true);
         if let Some(p) = &self.parent {
            if let Some(o) = p.upgrade() {
               o.base_state().request_draw();
            }
         }
      }
   }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
