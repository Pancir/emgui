use crate::{core::Painter, theme::Theme};
use m::Box2;
use std::rc::Rc;

////////////////////////////////////////////////////////////////////////////////////////////////////

/// Base render object type.
pub trait RenderObjectBase {
   /// Get the style type name for debugging purposes.
   ///
   /// # Note
   /// Developers should not override this method.
   #[inline]
   fn type_name(&self) -> &'static str {
      std::any::type_name::<Self>()
   }

   /// Get the style type name for debugging purposes.
   ///
   /// # Note
   /// Developers should not override this method.
   #[inline]
   fn type_name_short(&self) -> &'static str {
      let name = self.type_name();
      name.split('<').next().unwrap_or(name).split("::").last().unwrap_or(name)
   }

   /// To a strong reference counter.
   fn to_rc(self) -> Rc<Self>
   where
      Self: Sized,
   {
      Rc::new(self)
   }
}

/// Render object for certain data.
pub trait RenderObject<Data>: RenderObjectBase {
   /// Return render bounds.
   ///
   /// The render rectangle may actually be greatter or less than widget's geometry.
   fn rect(&self, data: &Data) -> Box2<f32>;

   /// Draw enabled widget.
   fn draw_enabled(&self, theme: &Theme, data: &Data, painter: &mut Painter);

   /// Draw disabled widget.
   fn draw_disabled(&self, theme: &Theme, data: &Data, painter: &mut Painter);
}

////////////////////////////////////////////////////////////////////////////////////////////////////
