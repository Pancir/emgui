use super::{ThemeColors, ThemeExtensions, ThemeFonts};
use crate::widgets::style::{ButtonStyle, ButtonStyleSheet};
use std::rc::Rc;

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct Theme {
   pub colors: ThemeColors,
   pub fonts: ThemeFonts,

   pub button: Rc<dyn ButtonStyleSheet>,

   pub extensions: ThemeExtensions,
}

impl Default for Theme {
   fn default() -> Self {
      Self {
         colors: ThemeColors::default(),
         fonts: ThemeFonts::default(),
         button: Rc::new(ButtonStyle::default()),
         extensions: Default::default(),
      }
   }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
