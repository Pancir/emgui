////////////////////////////////////////////////////////////////////////////////////////////////////

/// RGBA Color with float values.
#[repr(C)]
#[derive(PartialEq, Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Rgba {
   pub r: f32,
   pub g: f32,
   pub b: f32,
   pub a: f32,
}

impl Default for Rgba {
   fn default() -> Rgba {
      Self::BLACK
   }
}

impl From<Rgba8> for Rgba {
   fn from(v: Rgba8) -> Self {
      Self {
         r: v.r as f32 / 255.0,
         g: v.g as f32 / 255.0,
         b: v.b as f32 / 255.0,
         a: v.a as f32 / 255.0,
      }
   }
}

impl From<Argb8> for Rgba {
   fn from(v: Argb8) -> Self {
      Self {
         a: v.a as f32 / 255.0,
         r: v.r as f32 / 255.0,
         g: v.g as f32 / 255.0,
         b: v.b as f32 / 255.0,
      }
   }
}

impl Rgba {
   pub const fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
      Self { r, g, b, a }
   }

   pub const fn with_alpha(self, a: f32) -> Self {
      Self { a, ..self }
   }

   pub fn with_alpha_mul(mut self, a: f32) -> Self {
      self.r *= a;
      self.g *= a;
      self.b *= a;
      self
   }

   pub fn equal(self, other: Self, epsilon: f32) -> bool {
      (self.r - other.r).abs() < epsilon
         && (self.g - other.g).abs() < epsilon
         && (self.b - other.b).abs() < epsilon
         && (self.a - other.a).abs() < epsilon
   }

   pub fn lerp(self, other: Self, t: f32) -> Self {
      let t = t.clamp(0.0, 1.0);
      Self {
         r: m::core::lerp_precise(self.r, other.r, t),
         g: m::core::lerp_precise(self.g, other.g, t),
         b: m::core::lerp_precise(self.b, other.b, t),
         a: m::core::lerp_precise(self.a, other.a, t),
      }
   }
}

impl Rgba {
   pub const TRANSPARENT: Self = Self { r: 0.0, g: 0.0, b: 0.0, a: 0.0 };
   pub const BLACK: Self = Self { r: 0.0, g: 0.0, b: 0.0, a: 1.0 };
   pub const GRAY_DARK: Self = Self { r: 0.25, g: 0.25, b: 0.25, a: 1.0 };
   pub const GRAY: Self = Self { r: 0.5, g: 0.5, b: 0.5, a: 1.0 };
   pub const GRAY_LIGHT: Self = Self { r: 0.75, g: 0.75, b: 0.75, a: 1.0 };
   pub const WHITE: Self = Self { r: 1.0, g: 1.0, b: 1.0, a: 1.0 };
   pub const RED: Self = Self { r: 1.0, g: 0.0, b: 0.0, a: 1.0 };
   pub const GREEN: Self = Self { r: 0.0, g: 1.0, b: 0.0, a: 1.0 };
   pub const BLUE: Self = Self { r: 0.0, g: 0.0, b: 1.0, a: 1.0 };
   pub const CYAN: Self = Self { r: 0.0, g: 1.0, b: 1.0, a: 1.0 };
   pub const YELLOW: Self = Self { r: 1.0, g: 1.0, b: 0.0, a: 1.0 };
   pub const MAGENTA: Self = Self { r: 1.0, g: 0.0, b: 1.0, a: 1.0 };
   pub const AMBER: Self = Self { r: 1.0, g: 0.75, b: 0.0, a: 1.0 };
}

////////////////////////////////////////////////////////////////////////////////////////////////////

/// ARGB Color with u8 values.
#[repr(C)]
#[derive(PartialEq, Eq, Copy, Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Argb8 {
   pub a: u8,
   pub r: u8,
   pub g: u8,
   pub b: u8,
}

impl Default for Argb8 {
   fn default() -> Self {
      Self::BLACK
   }
}

impl From<Rgba8> for Argb8 {
   fn from(v: Rgba8) -> Self {
      Self { a: v.a, r: v.r, g: v.g, b: v.b }
   }
}

impl From<Argb8> for u32 {
   fn from(v: Argb8) -> u32 {
      ((v.a as u32) << 24) | ((v.r as u32) << 16) | ((v.g as u32) << 8) | (v.b as u32)
   }
}

impl From<Rgba> for Argb8 {
   fn from(v: Rgba) -> Self {
      Self {
         a: (255.0 * v.a) as u8,
         r: (255.0 * v.r) as u8,
         g: (255.0 * v.g) as u8,
         b: (255.0 * v.b) as u8,
      }
   }
}

impl From<u32> for Argb8 {
   fn from(v: u32) -> Self {
      Self::init(v)
   }
}

impl Argb8 {
   pub const fn new(a: u8, r: u8, g: u8, b: u8) -> Self {
      Self { a, r, g, b }
   }

   pub const fn with_alpha(self, a: u8) -> Self {
      Self { a, ..self }
   }

   pub const fn init(v: u32) -> Self {
      Self { a: (v >> 24) as u8, r: (v >> 16) as u8, g: (v >> 8) as u8, b: (v) as u8 }
   }

   pub const fn init_rgba(v: u32) -> Self {
      Self { r: (v >> 24) as u8, g: (v >> 16) as u8, b: (v >> 8) as u8, a: (v) as u8 }
   }
}

impl Argb8 {
   pub const TRANSPARENT: Self = Self { r: 0, g: 0, b: 0, a: 0 };
   pub const BLACK: Self = Self { r: 0, g: 0, b: 0, a: 255 };
   pub const GRAY_DARK: Self = Self { r: 64, g: 64, b: 64, a: 255 };
   pub const GRAY: Self = Self { r: 128, g: 128, b: 128, a: 255 };
   pub const GRAY_LIGHT: Self = Self { r: 191, g: 191, b: 191, a: 255 };
   pub const WHITE: Self = Self { r: 255, g: 255, b: 255, a: 255 };
   pub const RED: Self = Self { r: 255, g: 0, b: 0, a: 255 };
   pub const GREEN: Self = Self { r: 0, g: 255, b: 0, a: 255 };
   pub const BLUE: Self = Self { r: 0, g: 0, b: 255, a: 255 };
   pub const CYAN: Self = Self { r: 0, g: 255, b: 255, a: 255 };
   pub const YELLOW: Self = Self { r: 255, g: 255, b: 0, a: 255 };
   pub const MAGENTA: Self = Self { r: 255, g: 0, b: 255, a: 255 };
   pub const AMBER: Self = Self { r: 255, g: 191, b: 0, a: 255 };
}

////////////////////////////////////////////////////////////////////////////////////////////////////

/// RGBA Color with u8 values.
#[repr(C)]
#[derive(PartialEq, Eq, Copy, Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Rgba8 {
   pub r: u8,
   pub g: u8,
   pub b: u8,
   pub a: u8,
}

impl Default for Rgba8 {
   fn default() -> Self {
      Self::BLACK
   }
}

impl From<Argb8> for Rgba8 {
   fn from(v: Argb8) -> Self {
      Self { r: v.r, g: v.g, b: v.b, a: v.a }
   }
}

impl From<Rgba8> for u32 {
   fn from(v: Rgba8) -> u32 {
      ((v.r as u32) << 24) | ((v.g as u32) << 16) | ((v.b as u32) << 8) | (v.a as u32)
   }
}

impl From<Rgba> for Rgba8 {
   fn from(v: Rgba) -> Self {
      Self {
         r: (255.0 * v.r) as u8,
         g: (255.0 * v.g) as u8,
         b: (255.0 * v.b) as u8,
         a: (255.0 * v.a) as u8,
      }
   }
}

impl From<u32> for Rgba8 {
   fn from(v: u32) -> Self {
      Self::init(v)
   }
}

impl Rgba8 {
   pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
      Self { r, g, b, a }
   }

   pub const fn with_alpha(self, a: u8) -> Self {
      Self { a, ..self }
   }

   pub const fn init(v: u32) -> Self {
      Self { r: (v >> 24) as u8, g: (v >> 16) as u8, b: (v >> 8) as u8, a: (v) as u8 }
   }

   pub const fn init_argb(v: u32) -> Self {
      Self { a: (v >> 24) as u8, r: (v >> 16) as u8, g: (v >> 8) as u8, b: (v) as u8 }
   }
}

impl Rgba8 {
   pub const TRANSPARENT: Self = Self { r: 0, g: 0, b: 0, a: 0 };
   pub const BLACK: Self = Self { r: 0, g: 0, b: 0, a: 255 };
   pub const GRAY_DARK: Self = Self { r: 64, g: 64, b: 64, a: 255 };
   pub const GRAY: Self = Self { r: 128, g: 128, b: 128, a: 255 };
   pub const GRAY_LIGHT: Self = Self { r: 191, g: 191, b: 191, a: 255 };
   pub const WHITE: Self = Self { r: 255, g: 255, b: 255, a: 255 };
   pub const RED: Self = Self { r: 255, g: 0, b: 0, a: 255 };
   pub const GREEN: Self = Self { r: 0, g: 255, b: 0, a: 255 };
   pub const BLUE: Self = Self { r: 0, g: 0, b: 255, a: 255 };
   pub const CYAN: Self = Self { r: 0, g: 255, b: 255, a: 255 };
   pub const YELLOW: Self = Self { r: 255, g: 255, b: 0, a: 255 };
   pub const MAGENTA: Self = Self { r: 255, g: 0, b: 255, a: 255 };
   pub const AMBER: Self = Self { r: 255, g: 191, b: 0, a: 255 };
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
   use super::*;
   use float_eq::assert_float_eq;

   #[test]
   fn rgba8_u32_conversion() {
      let c = Rgba8::from(0x8A2BE2FF_u32);
      assert_eq!(c.r, 0x8A);
      assert_eq!(c.g, 0x2B);
      assert_eq!(c.b, 0xE2);
      assert_eq!(c.a, 0xFF);

      let u: u32 = c.into();
      assert_eq!(u, 0x8A2BE2FF_u32);
   }

   #[test]
   fn argb8_u32_conversion() {
      let c = Argb8::from(0xFF8A2BE2_u32);
      assert_eq!(c.r, 0x8A);
      assert_eq!(c.g, 0x2B);
      assert_eq!(c.b, 0xE2);
      assert_eq!(c.a, 0xFF);

      let u: u32 = c.into();
      assert_eq!(u, 0xFF8A2BE2_u32);
   }

   //------------------------------

   #[test]
   fn rgba8_argb8_conversion() {
      let rba8 = Rgba8::new(255, 128, 64, 32);

      let argb = Argb8::from(rba8);
      assert_eq!(argb.r, 255);
      assert_eq!(argb.g, 128);
      assert_eq!(argb.b, 64);
      assert_eq!(argb.a, 32);

      let rba8 = Rgba8::from(argb);
      assert_eq!(rba8.r, 255);
      assert_eq!(rba8.g, 128);
      assert_eq!(rba8.b, 64);
      assert_eq!(rba8.a, 32);
   }

   //------------------------------

   #[test]
   fn rgba8_rgba_conversion() {
      let c8 = Rgba8::new(255, 128, 64, 32);

      let cf = Rgba::from(c8);
      assert_float_eq!(cf.r, 001.0, abs <= 0.001);
      assert_float_eq!(cf.g, 000.5, abs <= 0.002);
      assert_float_eq!(cf.b, 00.25, abs <= 0.001);
      assert_float_eq!(cf.a, 0.125, abs <= 0.001);

      let c8 = Rgba8::from(cf);
      assert_eq!(c8.r, 255);
      assert_eq!(c8.g, 128);
      assert_eq!(c8.b, 64);
      assert_eq!(c8.a, 32);
   }

   #[test]
   fn argb8_rgba_conversion() {
      let c8 = Argb8::new(32, 255, 128, 64);

      let cf = Rgba::from(c8);
      assert_float_eq!(cf.r, 001.0, abs <= 0.001);
      assert_float_eq!(cf.g, 000.5, abs <= 0.002);
      assert_float_eq!(cf.b, 00.25, abs <= 0.001);
      assert_float_eq!(cf.a, 0.125, abs <= 0.001);

      let c8 = Argb8::from(cf);
      assert_eq!(c8.r, 255);
      assert_eq!(c8.g, 128);
      assert_eq!(c8.b, 64);
      assert_eq!(c8.a, 32);
   }

   //------------------------------

   #[test]
   fn rgba_equal() {
      let c1 = Rgba::new(0.55, 0.55, 0.55, 0.95);
      let c2 = Rgba::new(0.45, 0.45, 0.45, 01.0);
      assert!(c1.equal(c2, 0.2));
      assert!(!c1.equal(c2, 0.01));
   }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
