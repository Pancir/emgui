use std::{any::Any, borrow::Cow, rc::Rc};

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum SizePolicy {
   Pt,

   /// Size in pixels using full font height.
   PxFull,

   /// Size in pixels using ascend values only.
   PxAscend,
}

/// Font weight.
#[repr(u16)]
#[derive(Copy, Clone, PartialOrd, PartialEq, Debug, Default)]
pub enum FontWeight {
   /// Hairline (not official)
   Hairline = 1,

   /// Thin (Hairline)
   Thin = 100,

   /// Extra Light (Ultra Light)
   ExtraLight = 200,

   /// Light
   Light = 300,

   /// Normal (Regular)
   #[default]
   Normal = 400,

   /// News (not official)
   News = 450,

   /// Medium
   Medium = 500,

   /// Semi Bold (Demi Bold)
   SemiBold = 600,

   /// Bold
   Bold = 700,

   /// Extra Bold (Ultra Bold)
   ExtraBold = 800,

   /// Heavy (not official)
   Heavy = 850,

   /// Black (Heavy)
   Black = 900,

   /// Extra Black (Ultra Black) (not official)
   ExtraBlack = 950,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct Font {
   family: Cow<'static, str>,
   kerning: bool,
   weight: FontWeight,
   size_policy: SizePolicy,
   size: f32,
   face: Option<Rc<dyn Any>>,
}

impl Default for Font {
   fn default() -> Self {
      Self::new("Arial")
   }
}

impl Font {
   pub fn new<STR>(family: STR) -> Self
   where
      STR: Into<Cow<'static, str>>,
   {
      Self {
         family: family.into(),
         face: None,
         kerning: true,
         weight: FontWeight::default(),
         size_policy: SizePolicy::Pt,
         size: 32.0,
      }
   }

   #[inline]
   pub fn family(&self) -> &str {
      self.family.as_ref()
   }

   #[inline]
   pub fn set_kerning(&mut self, state: bool) {
      self.kerning = state;
   }

   #[inline]
   pub fn kerning(&self) -> bool {
      self.kerning
   }

   #[inline]
   pub fn set_size(&mut self, size: f32) {
      self.size = size;
      self.set_font_face_dirty();
   }

   #[inline]
   pub fn size(&self) -> f32 {
      self.size
   }

   #[inline]
   pub fn set_size_policy(&mut self, size_policy: SizePolicy) {
      self.size_policy = size_policy;
      self.set_font_face_dirty();
   }

   #[inline]
   pub fn size_policy(&self) -> SizePolicy {
      self.size_policy
   }

   #[inline]
   pub fn set_weight(&mut self, weight: FontWeight) {
      self.weight = weight;
      self.set_font_face_dirty();
   }

   pub fn weight(&self) -> FontWeight {
      self.weight
   }
}

impl Font {
   #[inline]
   pub fn font_face(&self) -> Option<Rc<dyn Any>> {
      self.face.clone()
   }

   #[inline]
   pub fn set_font_face_dirty(&mut self) {
      self.face = None
   }

   #[inline]
   pub fn set_font_face(&mut self, face: Rc<dyn Any>) {
      self.face = Some(face);
   }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
