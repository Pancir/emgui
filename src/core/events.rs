////////////////////////////////////////////////////////////////////////////////////////////////////

pub type KeyboardEventCtx = sim_run::KeyboardEvent;
pub type MouseButtonsEventCtx = sim_run::MouseButtonsEvent;
pub type MouseMoveEventCtx = sim_run::MouseMoveEvent;
pub type MouseWheelEventCtx = sim_run::MouseWheelEvent;
pub type UpdateEventCtx = sim_run::UpdateEvent;

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct LayoutEventCtx {}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub enum LifecycleEventCtx {
   /// The widget is going to be deleted after this event.
   Delete,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct DrawEventCtx {}

////////////////////////////////////////////////////////////////////////////////////////////////////
