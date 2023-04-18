use sim_draw::Canvas;

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct Painter {
   pub canvas: Box<Canvas>,
}

// TODO remove when code is ready for it
impl core::ops::Deref for Painter {
   type Target = Canvas;

   fn deref(&self) -> &Self::Target {
      &self.canvas
   }
}

// TODO remove when code is ready for it
impl core::ops::DerefMut for Painter {
   fn deref_mut(&mut self) -> &mut Self::Target {
      &mut self.canvas
   }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
