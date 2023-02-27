use crate::core::Theme;
use anyhow::bail;
use std::any::Any;

////////////////////////////////////////////////////////////////////////////////////////////////////

/// Storage for any application and user data.
///
/// The application data is stored in its own variable,
/// so the access to it is faster than access to the user data.
///
/// The user data is stored in the vector and every time
/// you request the one the vector is iterated to find it.
///
/// # Limitation
/// the struct can hold only one instance of the same type.
pub struct AppEnv {
   theme: Theme,
   app_data: Box<dyn Any>,
   user_data: Vec<Box<dyn Any>>,
}

impl AppEnv {
   /// Create a new with a general application data.
   pub fn new<APP: 'static>(app: APP, theme: Theme) -> Self {
      Self { app_data: Box::new(app), user_data: Vec::new(), theme }
   }
}

impl AppEnv {
   /// Get application data.
   pub fn app_data<APP: 'static>(&self) -> Option<&APP> {
      self.app_data.downcast_ref::<APP>()
   }

   /// Get mut application data.
   pub fn app_data_mut<APP: 'static>(&mut self) -> Option<&mut APP> {
      self.app_data.downcast_mut::<APP>()
   }

   /// Get application theme data.
   pub fn theme(&self) -> &Theme {
      &self.theme
   }
}

impl AppEnv {
   /// Remove user data.
   pub fn remove_data<USR: 'static>(&mut self) {
      // retain is not used to stop iterating when the type is found.
      let idx = self.user_data.iter().position(|v| (*v).downcast_ref::<USR>().is_some());
      if let Some(idx) = idx {
         self.user_data.remove(idx);
      }
   }

   /// Insert user data.
   ///
   /// # Return
   /// error if data with the specified type has already been inserted before.
   pub fn insert_data<USR: 'static>(&mut self, usr: USR) -> anyhow::Result<()> {
      if self.data::<USR>().is_some() {
         bail!("Type: [{}] has already been inserted before", std::any::type_name::<USR>());
      }

      self.user_data.push(Box::new(usr));
      Ok(())
   }

   /// Get user data by the specified type.
   pub fn data<USR: 'static>(&self) -> Option<&USR> {
      for data in &self.user_data {
         if let Some(d) = data.downcast_ref::<USR>() {
            return Some(d);
         }
      }
      None
   }

   /// Get mut user data by the specified type.
   pub fn data_mut<USR: 'static>(&mut self) -> Option<&mut USR> {
      for data in &mut self.user_data {
         if let Some(d) = data.downcast_mut::<USR>() {
            return Some(d);
         }
      }
      None
   }

   /// Remove all user data.
   pub fn clear(&mut self) {
      self.user_data.clear();
   }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
   use super::*;

   #[test]
   fn theme_size() {
      dbg!(std::mem::size_of::<AppEnv>());
   }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
