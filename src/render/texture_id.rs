////////////////////////////////////////////////////////////////////////////////////////////////////

/// Texture ID allocated in a backend.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct TextureId(i32);

impl Default for TextureId {
   fn default() -> Self {
      Self::INVALID
   }
}

impl TextureId {
   pub const INVALID: Self = Self::from_raw(-1);

   pub const fn raw(&self) -> i32 {
      self.0
   }

   pub fn raw_opt(&self) -> Option<i32> {
      if *self != Self::INVALID {
         Some(self.0)
      } else {
         None
      }
   }

   pub const fn from_raw(val: i32) -> Self {
      Self(val)
   }

   pub const fn is_valid(&self) -> bool {
      self.0 > -1
   }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
