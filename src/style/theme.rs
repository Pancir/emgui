use super::ThemeExtensions;
use crate::widgets::style::{ButtonStyle, ButtonStyleSheet};
use std::rc::Rc;

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct Theme {
   pub extensions: ThemeExtensions,

   pub button: Rc<dyn ButtonStyleSheet>,
}

impl Default for Theme {
   fn default() -> Self {
      Self { extensions: Default::default(), button: Rc::new(ButtonStyle {}) }
   }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
