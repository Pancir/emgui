use crate::render::TextureId;
use m::Box2;

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct Icon {
   geom: Box2<f32>,
   uv: Box2<f32>,
   texture: TextureId,
}

impl Default for Icon {
   fn default() -> Self {
      Self::new(Box2::UV_MAX, Box2::UV_MAX, TextureId::INVALID)
   }
}

impl Icon {
   #[must_use]
   #[inline]
   pub fn new(uv: Box2<f32>, geom: Box2<f32>, texture: TextureId) -> Self {
      Self { geom, uv, texture }
   }

   #[inline]
   pub fn geometry(&self) -> &Box2<f32> {
      &self.geom
   }

   #[inline]
   pub fn uv(&self) -> &Box2<f32> {
      &self.uv
   }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
