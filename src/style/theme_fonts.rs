use crate::core::Font;

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Default)]
pub struct ThemeFonts {
   pub user: Vec<Font>,
}

impl ThemeFonts {
   pub fn user<INDEX>(&self, index: INDEX) -> &Font
   where
      INDEX: Into<usize>,
   {
      &self.user[index.into()]
   }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
