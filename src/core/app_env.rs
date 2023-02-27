use crate::core::Theme;
use std::any::Any;

////////////////////////////////////////////////////////////////////////////////////////////////////

/// Storage for any application and user data.
///
/// The application data is stored in its own variable,
/// so the access to it is faster than access to the user data.
///
/// The user data is stored in the vector and every time
/// you request the one the vector is reverse iterated to find it.
///
/// Because it mostly designed to use in trees/gui/widgets,
/// the reverse iteration seems more effective as there is
/// big probability you need data from the end of the vector.
///
/// Limitation: the struct can hold only one instance of the same type.
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
      let idx = self.user_data.iter().position(|v| (*v).downcast_ref::<USR>().is_some());
      if let Some(idx) = idx {
         self.user_data.remove(idx);
      }
   }

   /// Insert user data.
   ///
   /// # Return
   /// error if data with the specified type has already been inserted before.
   pub fn insert_data<USR: 'static>(&mut self, usr: USR) -> std::result::Result<(), String> {
      if self.data::<USR>().is_some() {
         return Err(format!(
            "Type: [{}] has already been inserted before",
            std::any::type_name::<USR>()
         ));
      }

      self.user_data.push(Box::new(usr));
      Ok(())
   }

   /// Get user data by the specified type.
   pub fn data<USR: 'static>(&self) -> Option<&USR> {
      for data in self.user_data.iter().rev() {
         if let Some(d) = data.downcast_ref::<USR>() {
            return Some(d);
         }
      }
      None
   }

   /// Get mut user data by the specified type.
   pub fn data_mut<USR: 'static>(&mut self) -> Option<&mut USR> {
      for data in self.user_data.iter_mut().rev() {
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
