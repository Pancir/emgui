use sim_draw::m::{Box2};
use sim_draw::objects::UvRect;
use sim_draw::{Canvas, TextureId};

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct Icon {
   geom: UvRect,
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
      Self { geom: UvRect { rect: geom, uv }, texture }
   }

   #[inline]
   pub fn geometry(&self) -> &UvRect {
      &self.geom
   }
}

impl Icon {
   #[inline]
   pub fn on_draw(&self, canvas: &mut Canvas) {
      canvas.set_texture(Some(self.texture));
      canvas.tris(self.geometry())
   }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
