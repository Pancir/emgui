use crate::core::Geometry;

////////////////////////////////////////////////////////////////////////////////////////////////////

/// Common data for all widgets.
#[derive(Default)]
pub struct BaseState {
   pub geometry: Geometry,

   /// `True` if mouse over slider.
   pub is_hover: bool,

   /// `True` if the element is enabled and user can interact with it.
   pub is_enabled: bool,

   /// `True` if the element is enabled and user can interact with it.
   pub is_visible: bool,

   /// `True` if it is focused.
   pub has_focus: bool,

   /// `True` if element want draw event.
   pub needs_draw: bool,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
