use crate::core::{AppEnv, IWidget};
use std::cell::RefCell;
use std::fmt::Formatter;
use std::rc::Weak;

////////////////////////////////////////////////////////////////////////////////////////////////////

pub type KeyboardEventCtx = sim_run::KeyboardEvent;
pub type MouseButtonsEventCtx = sim_run::MouseButtonsEvent;
pub type MouseMoveEventCtx = sim_run::MouseMoveEvent;
pub type MouseWheelEventCtx = sim_run::MouseWheelEvent;

////////////////////////////////////////////////////////////////////////////////////////////////////

pub enum LifecycleEventCtx {
   /// When widget is placed into the heap a reference becomes available.
   ///
   /// You should not use it in the widget while an event is processed,
   /// because the widget is already borrowed in this case.
   /// The Reference can be used in another contexts including
   /// sending it somewhere else while interacting with the widget.
   SelfReference(Weak<RefCell<dyn IWidget>>),

   /// The widget is scheduled to be destroyed by dispatcher.
   ///
   /// unexpected `true` means that the widget will be destroyed in on incorrect situation,
   /// for example it can happen when dispatcher fails in an event and widget's children are lost.
   Destroy { unexpected: bool },
}

impl core::fmt::Debug for LifecycleEventCtx {
   fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
      match self {
         LifecycleEventCtx::SelfReference(_) => {
            f.debug_tuple("LifecycleEventCtx::SelfReference").field(&"ref").finish()
         }
         LifecycleEventCtx::Destroy { unexpected } => {
            f.debug_struct("LifecycleEventCtx::Destroy").field("unexpected", &unexpected).finish()
         }
      }
   }
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
   pub env: &'env mut AppEnv,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
