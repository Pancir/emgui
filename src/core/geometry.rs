use sim_draw::m::{Rect, Size};

////////////////////////////////////////////////////////////////////////////////////////////////////

/// Widget min/max size constraint.
#[derive(Copy, Clone)]
pub struct SizeConstraint {
   pub min: Size<f32>,
   pub max: Size<f32>,
}

impl Default for SizeConstraint {
   fn default() -> Self {
      Self {
         min: Size { width: 8.0, height: 8.0 },
         max: Size { width: f32::MAX, height: f32::MAX },
      }
   }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

/// Widget geometry and constraint.
#[derive(Copy, Clone)]
pub struct Geometry {
   rect: Rect<f32>,
   constraint: SizeConstraint,
}

impl Default for Geometry {
   fn default() -> Self {
      Self { rect: Rect::new(10.0, 10.0, 100.0, 60.0), constraint: SizeConstraint::default() }
   }
}

impl Geometry {
   /// Set new rect and ensure its size within the constraint.
   ///
   /// # Return
   /// true if rect was changed otherwise false.
   #[inline]
   pub fn set_rect(&mut self, rect: Rect<f32>) -> bool {
      if self.rect == rect {
         return false;
      }
      self.rect = rect;
      self.ensure_max_size();
      self.ensure_min_size();
      return true;
   }

   /// Get rectangle.
   #[inline]
   pub fn rect(&self) -> Rect<f32> {
      self.rect
   }

   /// Set new constraint and ensure the rect size within the constraint.
   ///
   /// # Return
   /// true if rect was changed otherwise false.
   #[inline]
   pub fn set_size_constraint(&mut self, c: SizeConstraint) -> bool {
      self.constraint = c;
      self.ensure_max_size() || self.ensure_min_size()
   }

   /// Get constraint.
   #[inline]
   pub fn constraint(&self) -> &SizeConstraint {
      &self.constraint
   }

   /// Set max size constraint and ensure the rect size within the constraint.
   ///
   /// # Return
   /// true if rect was changed otherwise false.
   #[inline]
   pub fn set_max_size(&mut self, size: Size<f32>) -> bool {
      self.constraint.max = size;
      self.ensure_max_size()
   }

   /// Set min size constraint and ensure the rect size within the constraint.
   ///
   /// # Return
   /// true if rect was changed otherwise false.
   #[inline]
   pub fn set_min_size(&mut self, size: Size<f32>) -> bool {
      self.constraint.min = size;
      self.ensure_min_size()
   }
}

impl Geometry {
   fn ensure_max_size(&mut self) -> bool {
      let size: Size<f32> = self.rect.into();
      self.rect.width = self.rect.width.min(self.constraint.max.width);
      self.rect.height = self.rect.height.min(self.constraint.max.height);
      size != self.rect.into()
   }

   fn ensure_min_size(&mut self) -> bool {
      let size: Size<f32> = self.rect.into();
      self.rect.width = self.rect.width.max(self.constraint.min.width);
      self.rect.height = self.rect.height.max(self.constraint.min.height);
      size != self.rect.into()
   }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
   use super::*;

   #[test]
   fn sizes() {
      dbg!(std::mem::size_of::<Geometry>());
      dbg!(std::mem::size_of::<SizeConstraint>());
   }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
