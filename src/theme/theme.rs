use super::{style::StyleBase, ThemeColors, ThemeElements, ThemeExtensions, ThemeFonts};
use crate::widgets::style::{ButtonStyle, ButtonStyleSheet};

////////////////////////////////////////////////////////////////////////////////////////////////////

pub enum ButtonDefined {
   Normal,
   Accent,
}

impl Into<&'static str> for ButtonDefined {
   fn into(self) -> &'static str {
      match self {
         ButtonDefined::Normal => "normal",
         ButtonDefined::Accent => "accent",
      }
   }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct Theme {
   pub colors: ThemeColors,
   pub fonts: ThemeFonts,

   pub buttons: ThemeElements<dyn ButtonStyleSheet>,

   pub extensions: ThemeExtensions,
}

impl Default for Theme {
   fn default() -> Self {
      let mut buttons = ThemeElements::<dyn ButtonStyleSheet>::new(3);
      buttons
         .register_multi([
            (ButtonDefined::Normal, ButtonStyle::new_normal().to_rc()),
            (ButtonDefined::Accent, ButtonStyle::new_accent().to_rc()),
         ])
         .unwrap();

      Self {
         colors: ThemeColors::default(),
         fonts: ThemeFonts::default(),
         buttons,
         extensions: Default::default(),
      }
   }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
