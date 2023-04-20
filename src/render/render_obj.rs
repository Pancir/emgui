use super::Painter;
use crate::{core::upcast_rc, theme::Theme};
use m::Box2;
use std::{any::Any, rc::Rc};

////////////////////////////////////////////////////////////////////////////////////////////////////

/// Base render object type.
pub trait RenderObjectBase: Any + upcast_rc::Upcast<dyn Any> {
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

impl<'a, T: Any + 'a> upcast_rc::UpcastFrom<T> for dyn Any + 'a {
   fn up_from_rc(value: Rc<T>) -> Rc<Self> {
      value
   }
}

/// Render object for certain data.
pub trait RenderObject<Data>: RenderObjectBase {
   /// Return render bounds.
   ///
   /// The render rectangle may actually be greater or less than widget's geometry.
   fn rect(&self, data: &Data) -> Box2<f32>;

   /// Draw widget.
   fn draw(&self, theme: &Theme, data: &Data, painter: &mut Painter);
}

////////////////////////////////////////////////////////////////////////////////////////////////////
