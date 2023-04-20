use super::{
   input::{
      keyboard::KeyboardInput,
      mouse::{MouseButtonsInput, MouseMoveInput, MouseWheelInput},
   },
   WidgetRef,
};
use crate::core::AppEnv;
use m::Rect;
use std::{fmt::Formatter, time::Duration};

////////////////////////////////////////////////////////////////////////////////////////////////////

pub enum LifecycleState {
   /// After a new runtime set into the [WidgetBase] associated with the widget.
   ///
   /// You can access to the runtime using your base [WidgetBase::runtime] method.
   RuntimeSet,
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
         Self::RuntimeSet => f.debug_struct("LifecycleEventCtx::RuntimeSet").finish(),
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
   pub call_num: usize,
   pub raw_delta: f64,
   pub raw_abs_time: f64,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct DrawEventCtx<'env> {
   pub abs_time: Duration,
   pub region: Option<Rect<f32>>,
   pub env: &'env mut AppEnv,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct KeyboardEventCtx {
   pub input: KeyboardInput,
}

pub struct MouseButtonsEventCtx {
   pub input: MouseButtonsInput,
}

pub struct MouseMoveEventCtx {
   pub input: MouseMoveInput,
}

pub struct MouseWheelEventCtx {
   pub input: MouseWheelInput,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
