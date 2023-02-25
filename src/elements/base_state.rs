use crate::core::Geometry;
use crate::widgets::IWidget;
use std::cell::Cell;
use std::rc::Weak;

////////////////////////////////////////////////////////////////////////////////////////////////////

/// Common data for all widgets.
#[derive(Default)]
pub struct BaseState {
   /// Element geometry.
   pub geometry: Geometry,

   /// Element's parent.
   pub parent: Option<Weak<dyn IWidget>>,

   /// Self element reference.
   pub self_ref: Option<Weak<dyn IWidget>>,

   /// `True` if mouse over slider.
   pub is_hover: bool,

   /// `True` if the element is enabled and user can interact with it.
   pub is_enabled: bool,

   /// `True` if the element is enabled and user can interact with it.
   pub is_visible: bool,

   /// `True` if it is focused.
   pub has_focus: bool,

   //---------------------------
   /// `True` if element want draw event.
   needs_draw: Cell<bool>,
}

impl BaseState {
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
