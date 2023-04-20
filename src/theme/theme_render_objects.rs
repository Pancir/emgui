use crate::render::RenderObjectBase;
use anyhow::bail;
use std::rc::Rc;

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct ThemeRenderObjects<T>
where
   T: RenderObjectBase + ?Sized,
{
   data: Vec<(&'static str, Rc<T>)>,
}

impl<T> ThemeRenderObjects<T>
where
   T: RenderObjectBase + ?Sized,
{
   #[inline]
   pub fn new(capacity: usize) -> Self {
      Self { data: Vec::with_capacity(capacity) }
   }

   #[inline]
   pub fn register_multi<N, const NUM: usize>(
      &mut self,
      data: [(N, Rc<T>); NUM],
   ) -> anyhow::Result<()>
   where
      N: Into<&'static str>,
   {
      for d in data {
         self.register(d.0.into(), d.1)?;
      }

      Ok(())
   }

   #[inline]
   pub fn register(&mut self, name: &'static str, style: Rc<T>) -> anyhow::Result<()> {
      if let Some(i) = self.data.iter().find(|v| v.0 == name) {
         bail!(
            "A style with the name <{}> has already been registered for type <{}>",
            name,
            i.1.type_name()
         )
      }

      self.data.push((name.into(), style));
      Ok(())
   }

   #[inline]
   pub fn get(&self, name: &'static str) -> Option<Rc<T>> {
      self.data.iter().find(|v| v.0 == name).map(|v| v.1.clone())
   }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
