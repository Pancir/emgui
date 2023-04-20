////////////////////////////////////////////////////////////////////////////////////////////////////

// FIXME use from layout

#[repr(u8)]
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AlignV {
   Top,
   Middle,
   Bottom,

   /// It is mostly used for fonts.
   ///
   /// In layouts it is usually the same as [Self::Bottom].
   BaseLine,
}

#[repr(u8)]
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AlignH {
   Left,
   Center,
   Right,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

/// Text align properties.
///
/// # Default
/// * [crate::layout::AlignH::Right]
/// * [crate::layout::AlignV::BaseLine]
#[derive(Copy, Clone, Debug)]
pub struct TextAlign {
   tight: bool,
   h_a: AlignH,
   v_a: AlignV,
}

impl Default for TextAlign {
   #[inline]
   fn default() -> Self {
      Self::new()
   }
}

impl TextAlign {
   #[inline]
   #[must_use]
   pub const fn new() -> Self {
      Self { tight: false, h_a: AlignH::Right, v_a: AlignV::BaseLine }
   }

   #[inline]
   #[must_use]
   pub const fn left(self) -> Self {
      self.h_align(AlignH::Left)
   }

   #[inline]
   #[must_use]
   pub const fn center(self) -> Self {
      self.h_align(AlignH::Center)
   }

   #[inline]
   #[must_use]
   pub const fn right(self) -> Self {
      self.h_align(AlignH::Right)
   }

   #[inline]
   #[must_use]
   pub const fn top(self) -> Self {
      self.v_align(AlignV::Top)
   }

   #[inline]
   #[must_use]
   pub const fn middle(self) -> Self {
      self.v_align(AlignV::Middle)
   }

   #[inline]
   #[must_use]
   pub const fn bottom(self) -> Self {
      self.v_align(AlignV::Bottom)
   }

   #[inline]
   #[must_use]
   pub const fn baseline(self) -> Self {
      self.v_align(AlignV::BaseLine)
   }

   /// Use actual text geometry to align instead of the font metrics.
   ///
   /// For example it is useful to align text inside a button, if
   /// the font metrics is used the text may be a bit offset.
   #[inline]
   #[must_use]
   pub const fn tight(mut self) -> Self {
      self.tight = true;
      self
   }

   /// Horizontal align.
   #[inline]
   #[must_use]
   pub const fn h_align(mut self, h_a: AlignH) -> Self {
      self.h_a = h_a;
      self
   }

   /// Vertical align.
   #[inline]
   #[must_use]
   pub const fn v_align(mut self, v_a: AlignV) -> Self {
      self.v_a = v_a;
      self
   }

   /// Get values.
   #[inline]
   #[must_use]
   pub const fn get(&self) -> (bool, AlignH, AlignV) {
      (self.tight, self.h_a, self.v_a)
   }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
