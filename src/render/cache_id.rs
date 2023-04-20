////////////////////////////////////////////////////////////////////////////////////////////////////

/// Render object cache id.
///
/// Widgets can hold this id for render object entities.
/// The cache mechanism can be implemented on user side,
/// thid library does not have one. So user is responsible
/// to implement caching.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct CacheId(u32);

impl Default for CacheId {
   fn default() -> Self {
      Self::INVALID
   }
}

impl CacheId {
   pub const INVALID: Self = Self::from_raw(u32::MAX);

   pub const fn raw(&self) -> u32 {
      self.0
   }

   pub fn raw_opt(&self) -> Option<u32> {
      if *self != Self::INVALID {
         Some(self.0)
      } else {
         None
      }
   }

   pub const fn from_raw(val: u32) -> Self {
      Self(val)
   }

   pub const fn is_valid(&self) -> bool {
      self.0 != Self::INVALID.0
   }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
