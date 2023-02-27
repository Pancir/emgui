////////////////////////////////////////////////////////////////////////////////////////////////////

/// Unique within application instance widget id.
#[derive(Copy, Clone, Debug, Hash)]
pub struct WidgetId((usize, &'static str));

impl WidgetId {
   pub const INVALID: Self = Self((0, "none"));
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
      Self((
         WIDGET_ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
         std::any::type_name::<T>(),
      ))
   }

   pub fn from_raw<T>(v: usize) -> Self {
      Self((v, std::any::type_name::<T>()))
   }

   pub fn raw(&self) -> usize {
      self.0 .0
   }

   pub fn is_valid(self) -> bool {
      self != Self::INVALID
   }
}

impl PartialEq for WidgetId {
   fn eq(&self, other: &Self) -> bool {
      self.0 == other.0
   }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
