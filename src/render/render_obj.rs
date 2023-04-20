use super::Canvas;
use crate::{core::upcast_rc, theme::Theme};
use m::Box2;
use std::{
   any::{Any, TypeId},
   rc::Rc,
};

////////////////////////////////////////////////////////////////////////////////////////////////////

/// Base render object type.
///
/// As it is almost imposable to downcast to another trait object in the Rust (in 2023-04-20),
/// this entity has 3 functions that make it more universal to use.
/// * [Self::can_render]
/// * [Self::render_any_bounds]
/// * [Self::render_any]
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

   //-----------------------------------------------

   /// Check whether the specified data type can be rendered by this render object.
   fn can_render(&self, type_id: TypeId) -> bool;

   /// Render bounds
   ///
   /// `None` it input data is not supported.
   fn render_any_bounds(&self, theme: &Theme, data: &dyn Any) -> Option<Box2<f32>>;

   /// Render data.
   ///
   /// # Return
   /// `Ok` if the specified data is not supported by this render object.
   fn render_any(&self, theme: &Theme, data: &dyn Any, canvas: &mut Canvas) -> anyhow::Result<()>;
}

impl<'a, T: Any + 'a> upcast_rc::UpcastFrom<T> for dyn Any + 'a {
   fn up_from_rc(value: Rc<T>) -> Rc<Self> {
      value
   }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

/// Render object for certain data.
pub trait RenderObject<Data>: RenderObjectBase {
   /// Return render bounds.
   ///
   /// The render rectangle may actually be greater or less than widget's geometry.
   fn render_bounds(&self, theme: &Theme, data: &Data) -> Box2<f32>;

   /// Draw widget.
   fn render(&self, theme: &Theme, data: &Data, canvas: &mut Canvas);
}

////////////////////////////////////////////////////////////////////////////////////////////////////
