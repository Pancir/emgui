////////////////////////////////////////////////////////////////////////////////////////////////////

/// Unique within application instance widget id.
#[derive(Default, Copy, Clone, Debug, Hash, PartialEq)]
pub struct WidgetId(usize);

impl WidgetId {
   pub const INVALID: Self = Self(0);
}

impl WidgetId {
   pub fn new() -> Self {
      static WIDGET_ID_COUNTER: std::sync::atomic::AtomicUsize =
         std::sync::atomic::AtomicUsize::new(1);
      WidgetId(WIDGET_ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed))
   }

   pub fn is_valid(self) -> bool {
      self != Self::INVALID
   }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
