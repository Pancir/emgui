////////////////////////////////////////////////////////////////////////////////////////////////////

/// Unique within application instance widget id.
#[derive(Copy, Clone, Debug, Hash)]
pub struct WidgetId {
   id: usize,

   #[cfg(feature = "widget-id-type")]
   type_name: &'static str,
}

impl WidgetId {
   #[cfg(feature = "widget-id-type")]
   pub const INVALID: Self = Self { id: 0, type_name: "none" };

   #[cfg(not(feature = "widget-id-type"))]
   pub const INVALID: Self = Self { id: 0 };
}

impl Default for WidgetId {
   fn default() -> Self {
      Self::INVALID
   }
}

impl WidgetId {
   pub fn new<T>() -> Self {
      static WIDGET_ID_COUNTER: std::sync::atomic::AtomicUsize =
         std::sync::atomic::AtomicUsize::new(1);

      #[cfg(feature = "widget-id-type")]
      {
         Self {
            id: WIDGET_ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            type_name: std::any::type_name::<T>(),
         }
      }

      #[cfg(not(feature = "widget-id-type"))]
      {
         Self { id: WIDGET_ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed) }
      }
   }

   pub fn from_raw<T>(v: usize) -> Self {
      #[cfg(feature = "widget-id-type")]
      {
         Self { id: v, type_name: std::any::type_name::<T>() }
      }

      #[cfg(not(feature = "widget-id-type"))]
      {
         Self { id: v }
      }
   }

   pub fn raw(&self) -> usize {
      self.id
   }

   pub fn is_valid(self) -> bool {
      self.id != Self::INVALID.id
   }
}

impl PartialEq for WidgetId {
   fn eq(&self, other: &Self) -> bool {
      self.id == other.id
   }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
