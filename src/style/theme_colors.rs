use sim_draw::color::Rgba;

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Default)]
pub struct ThemeColors {
   /// Use your custom enum to access by an [index](Self::user).
   ///
   /// ```
   /// use sim_draw::color::Rgba;
   /// use sim_ui::style::ThemeColors;
   ///
   /// #[repr(usize)]
   /// enum MyColors {
   ///    RED,
   ///    WHITE,
   /// }
   ///
   /// impl core::convert::Into<usize> for MyColors {
   ///    fn into(self) -> usize {
   ///       self as usize
   ///    }
   /// }
   ///
   /// let colors = ThemeColors { user: vec![Rgba::RED, Rgba::WHITE] , ..ThemeColors::default()};
   /// print!("{:?}", colors.user(MyColors::RED))
   ///
   /// ```
   pub user: Vec<Rgba>,
}

impl ThemeColors {
   pub fn user<INDEX>(&self, index: INDEX) -> Rgba
   where
      INDEX: Into<usize>,
   {
      self.user[index.into()]
   }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
