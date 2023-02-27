use crate::core::ThemeExtensions;

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Default)]
pub struct Theme {
   pub extensions: ThemeExtensions,
}

impl Theme {
   pub fn default_dark() -> Self {
      Self { extensions: Default::default() }
   }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
   use super::*;

   #[test]
   fn theme_size() {
      dbg!(std::mem::size_of::<Theme>());
   }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
