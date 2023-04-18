use super::WidgetRef;
use crate::core::AppEnv;
use sim_draw::m::Rect;
use std::{fmt::Formatter, time::Duration};

////////////////////////////////////////////////////////////////////////////////////////////////////

pub type KeyboardEventCtx = sim_run::KeyboardEvent;
pub type MouseButtonsEventCtx = sim_run::MouseButtonsEvent;
pub type MouseMoveEventCtx = sim_run::MouseMoveEvent;
pub type MouseWheelEventCtx = sim_run::MouseWheelEvent;

////////////////////////////////////////////////////////////////////////////////////////////////////

pub enum LifecycleState {
   /// When widget is placed into the heap a reference becomes available.
   ///
   /// You should not use it in the widget while an event is processed,
   /// because the widget is already borrowed in this case.
   /// The Reference can be used in another contexts including
   /// sending it somewhere else while interacting with the widget.
   SelfReference(WidgetRef),

   /// The widget is scheduled to be destroyed by dispatcher.
   ///
   /// unexpected `true` means that the widget will be destroyed in on incorrect situation,
   /// for example it can happen when dispatcher fails in an event and widget's children are lost.
   Destroy { unexpected: bool },
}

impl core::fmt::Debug for LifecycleState {
   fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
      match self {
         Self::SelfReference(_) => {
            f.debug_tuple("LifecycleEventCtx::SelfReference").field(&"ref").finish()
         }
         Self::Destroy { unexpected } => {
            f.debug_struct("LifecycleEventCtx::Destroy").field("unexpected", &unexpected).finish()
         }
      }
   }
}

pub struct LifecycleEventCtx {
   pub state: LifecycleState,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct LayoutEventCtx {}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct UpdateEventCtx<'a> {
   pub env: &'a mut AppEnv,
   pub data: &'a sim_run::UpdateEvent,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct DrawEventCtx<'env> {
   pub abs_time: Duration,
   pub region: Option<Rect<f32>>,
   pub env: &'env mut AppEnv,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
