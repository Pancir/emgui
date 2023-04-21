use crate::render::ImageId;
use m::Box2;

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct Icon {
   geom: Box2<f32>,
   uv: Box2<f32>,
   texture: ImageId,
}

impl Default for Icon {
   fn default() -> Self {
      Self::new(Box2::UV_MAX, Box2::UV_MAX, ImageId::INVALID)
   }
}

impl Icon {
   #[must_use]
   #[inline]
   pub fn new(uv: Box2<f32>, geom: Box2<f32>, texture: ImageId) -> Self {
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
