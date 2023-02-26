////////////////////////////////////////////////////////////////////////////////////////////////////

use crate::core::IWidget;
use std::cell::RefCell;
use std::rc::Weak;

pub type KeyboardEventCtx = sim_run::KeyboardEvent;
pub type MouseButtonsEventCtx = sim_run::MouseButtonsEvent;
pub type MouseMoveEventCtx = sim_run::MouseMoveEvent;
pub type MouseWheelEventCtx = sim_run::MouseWheelEvent;
pub type UpdateEventCtx = sim_run::UpdateEvent;

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct LayoutEventCtx {}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub enum LifecycleEventCtx {
   /// When widget is placed into the heap a reference becomes available.
   ///
   /// You should not use it in the widget while an event is processed,
   /// because the widget is already borrowed in this case.
   /// The Reference can be used in another contexts including
   /// sending it somewhere else while interacting with the widget.
   SelfReference(Weak<RefCell<dyn IWidget>>),

   /// The widget is scheduled to be deleted by dispatcher.
   ///
   /// unexpected `true` means that the widget will be deleted in on incorrect situation,
   /// for example it can happen when dispatcher fails in an event and widget's children are lost.
   Delete { unexpected: bool },
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct DrawEventCtx {}

////////////////////////////////////////////////////////////////////////////////////////////////////
