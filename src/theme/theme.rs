use super::{ThemeColors, ThemeExtensions, ThemeFonts, ThemeRenderObjects};
use crate::{
   render::RenderObjectBase,
   widgets::render::{ButtonRender, ButtonRenderObject},
};

////////////////////////////////////////////////////////////////////////////////////////////////////

pub enum ButtonDefined {
   Normal,
   Accent,
}

#[allow(clippy::from_over_into)]
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

   pub buttons: ThemeRenderObjects<dyn ButtonRenderObject>,

   pub extensions: ThemeExtensions,
}

impl Default for Theme {
   fn default() -> Self {
      let mut buttons = ThemeRenderObjects::<dyn ButtonRenderObject>::new(3);
      buttons
         .register_multi([
            (ButtonDefined::Normal, ButtonRender::new_normal().to_rc()),
            (ButtonDefined::Accent, ButtonRender::new_accent().to_rc()),
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
