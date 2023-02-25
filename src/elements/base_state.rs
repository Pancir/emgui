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

   /// `True` if mouse over slider.
   pub is_hover: bool,

   /// `True` if the element is enabled and user can interact with it.
   pub is_enabled: bool,

   /// `True` if the element is enabled and user can interact with it.
   pub is_visible: bool,

   /// `True` if it is focused.
   pub has_focus: bool,

   /// `True` if element want draw event.
   pub needs_draw: Cell<bool>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
