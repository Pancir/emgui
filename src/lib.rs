//! Contract:
//! 1. Widget which is being processed does not have children list to read, so
//!    if you need to do some modification for some children in that moment,
//!    your widget should save it separately somewhere. The widget itself
//!    can be modified in this moment. Usually this problem can be in parent
//!    widget relative to the current being processed.
//!    TODO examples
//! 2. TODO description/examples about loop call, when you do something on mouse click
//!    and this call returns to the widget  and tries to modify it. Example:
//!    `button->click->controller-> ... ->(change text) button` The button is
//!     being processed at this moment and can be modified outside.

////////////////////////////////////////////////////////////////////////////////////////////////////

pub mod core;
pub mod defines;
pub mod elements;
pub mod theme;
// pub mod experiments;
pub mod widgets;

////////////////////////////////////////////////////////////////////////////////////////////////////
