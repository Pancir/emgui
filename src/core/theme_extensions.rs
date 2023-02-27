use std::any::Any;

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Default)]
pub struct ThemeExtensions {
   extensions: Vec<Box<dyn Any>>,
}

impl ThemeExtensions {
   pub fn remove<T: 'static>(&mut self) {
      let idx = self.extensions.iter().position(|v| (*v).downcast_ref::<T>().is_some());
      if let Some(idx) = idx {
         self.extensions.remove(idx);
      }
   }

   pub fn insert<T: 'static>(&mut self, data: T) -> std::result::Result<(), String> {
      if self.get::<T>().is_some() {
         return Err(format!(
            "Type: [{}] has already been inserted before",
            std::any::type_name::<T>()
         ));
      }

      self.extensions.push(Box::new(data));
      Ok(())
   }

   pub fn get<T: 'static>(&self) -> Option<&T> {
      for data in self.extensions.iter().rev() {
         if let Some(d) = data.downcast_ref::<T>() {
            return Some(d);
         }
      }
      None
   }

   pub fn get_mut<T: 'static>(&mut self) -> Option<&mut T> {
      for data in self.extensions.iter_mut().rev() {
         if let Some(d) = data.downcast_mut::<T>() {
            return Some(d);
         }
      }
      None
   }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
