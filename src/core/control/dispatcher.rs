use super::*;

use crate::core::control::runtime::Runtime;
use crate::core::events::{
   DrawEventCtx, KeyboardEventCtx, LayoutEventCtx, LifecycleEventCtx, LifecycleState,
   MouseButtonsEventCtx, MouseMoveEventCtx, MouseWheelEventCtx, UpdateEventCtx,
};
use crate::core::{AppEnv, IWidget, Widget};
use sim_draw::Canvas;
use sim_run::UpdateEvent;
use std::cell::RefCell;
use std::rc::Rc;

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct InnerDispatcher {
   widget_mouse_over: Option<Weak<RefCell<dyn IWidget>>>,
   runtime: Runtime,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct Dispatcher {
   inner: InnerDispatcher,
   root: Rc<RefCell<dyn IWidget>>,
   destroyed: bool,
}

impl Dispatcher {
   #[inline]
   pub fn new(root: Option<Rc<RefCell<dyn IWidget>>>) -> Self {
      let mut out = Self {
         inner: InnerDispatcher { runtime: Runtime::new(), widget_mouse_over: None },
         root: root.unwrap_or_else(|| Widget::new(|_| (), |_| ())),
         destroyed: false,
      };

      out.set_runtime_to_widget();
      out
   }

   #[inline]
   pub fn reinit(&mut self, root: Rc<RefCell<dyn IWidget>>) {
      if !self.destroyed {
         self.destroy();
      }
      self.root = root;
      self.set_runtime_to_widget();
   }

   #[inline]
   pub fn widget(&self) -> &Rc<RefCell<dyn IWidget>> {
      &self.root
   }
}

impl std::ops::Drop for Dispatcher {
   fn drop(&mut self) {
      if !self.destroyed {
         log::warn!(
            "You forgot to call destroy on dispatcher. It is auto-called while dropping, \
but it may lead to unexpected behaviour because it can be too late, please call it yourself."
         );

         self.destroy()
      }
   }
}

impl Dispatcher {
   pub fn destroy(&mut self) {
      if self.destroyed {
         log::warn!("Attempt to call destroy on destroyed dispatcher.");
         return;
      }
      self.destroyed = true;
      Self::emit_inner_destroy(&mut self.inner, &self.root);
   }

   fn emit_inner_destroy(dispatcher: &mut InnerDispatcher, child: &Rc<RefCell<dyn IWidget>>) {
      Self::emit_inner_lifecycle(
         dispatcher,
         &child,
         &LifecycleEventCtx { state: LifecycleState::Destroy { unexpected: false } },
      );
   }
}

impl Dispatcher {
   fn set_runtime_to_widget(&mut self) {
      Self::set_runtime_to_widget_inner(self.inner.runtime.clone(), &self.root);
   }

   fn set_runtime_to_widget_inner(runtime: Runtime, child: &Rc<RefCell<dyn IWidget>>) {
      match child.try_borrow_mut() {
         Ok(mut bor) => {
            let mut internal = bor.internal_mut();
            internal.runtime = Some(runtime.clone());

            let children = internal.take_children(internal.id);
            for child in &children {
               Self::set_runtime_to_widget_inner(runtime.clone(), &child);
            }
            internal.set_children(children, internal.id)
         }
         Err(e) => panic!("{}", e),
      }
   }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

impl Dispatcher {
   #[cfg_attr(feature = "trace-dispatcher", tracing::instrument(level = "trace", skip_all))]
   fn emit_inner_lifecycle(
      dispatcher: &mut InnerDispatcher,
      child: &Rc<RefCell<dyn IWidget>>,
      event: &LifecycleEventCtx,
   ) {
      //--------------------------------------------------
      // # Safety
      // It seems it is quite safe, we just read simple copiable variables.
      // Just in case in debug mode we check availability.
      debug_assert!(child.try_borrow_mut().is_ok());
      let id = unsafe {
         let internal = (*child.as_ptr()).internal();
         internal.id
      };
      //---------------------------------
      let children = match child.try_borrow_mut() {
         Ok(mut child) => child.internal_mut().take_children(id),
         Err(e) => {
            log::error!("Can't borrow widget [{:?}] to process lifecycle event!\n\t{}", id, e);
            return;
         }
      };
      //--------------------------------------------------
      if !children.is_empty() {
         for child in &children {
            Self::emit_inner_lifecycle(dispatcher, &child, event);
         }
      }
      //--------------------------------------------------
      match child.try_borrow_mut() {
         Ok(mut child) => {
            child.internal_mut().set_children(children, id);
            child.emit_lifecycle(event);
         }
         Err(e) => {
            log::error!(
               "Can't borrow widget [{:?}] to process lifecycle finalization! Children [{}] ARE LOST!\n\t{}",
               id,
               children.len(),
               e
            );
         }
      };
   }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

impl Dispatcher {
   #[cfg_attr(feature = "trace-dispatcher", tracing::instrument(level = "trace", skip_all))]
   pub fn emit_layout(&mut self, event: &LayoutEventCtx) {
      Self::emit_inner_layout(&mut self.inner, &self.root, event);
   }

   fn emit_inner_layout(
      dispatcher: &mut InnerDispatcher,
      child: &Rc<RefCell<dyn IWidget>>,
      event: &LayoutEventCtx,
   ) {
      let children = match child.try_borrow_mut() {
         Ok(mut child) => {
            child.emit_layout(event);
            let id = child.id();
            child.internal_mut().take_children(id)
         }
         Err(e) => {
            panic!("{}", e)
         }
      };
      //--------------------------------------------------
      if !children.is_empty() {
         for child in &children {
            Self::emit_inner_layout(dispatcher, &child, event);
         }
      }
      //--------------------------------------------------
      let mut bor = child.try_borrow_mut().unwrap_or_else(|e| panic!("{}", e));
      let id = bor.id();
      bor.internal_mut().set_children(children, id);
   }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

impl Dispatcher {
   /// This event check if there are widgets to delete and perform deleting.
   #[cfg_attr(feature = "trace-dispatcher", tracing::instrument(level = "trace", skip_all))]
   fn emit_inner_check_delete(
      dispatcher: &mut InnerDispatcher,
      input_child: &Rc<RefCell<dyn IWidget>>,
   ) {
      //--------------------------------------------------
      // # Safety
      // It seems it is quite safe, we just read simple copiable variables.
      // Just in case in debug mode we check availability.
      debug_assert!(input_child.try_borrow_mut().is_ok());
      let (state_flags, id) = unsafe {
         let internal = (*input_child.as_ptr()).internal();
         (internal.state_flags.get(), internal.id)
      };
      //---------------------------------
      #[cfg(debug_assertions)]
      {
         // Only root widget can have self delete flag and it is should be
         // processed by this function caller.

         if state_flags.contains(StateFlags::SELF_DELETE) {
            log::error!(
               "Not expected flag [{:?}] in [{:?}] with all flags: [{:?}]. \
               Possible problem that a root widget is market as self delete!",
               StateFlags::SELF_DELETE,
               id,
               state_flags
            )
         }
      }
      //--------------------------------------------------
      if state_flags.contains(StateFlags::CHILDREN_DELETE) {
         let mut children = match input_child.try_borrow_mut() {
            Ok(mut child) => child.internal_mut().take_children(id),
            Err(e) => {
               log::error!("Can't borrow widget [{:?}] to process delete event!\n\t{}", id, e);
               return;
            }
         };
         //-------------------------------------------
         children.retain(|child| {
            // # Safety
            // It seems it is quite safe, we just read simple copiable variables.
            // Just in case in debug mode we check availability.
            debug_assert!(child.try_borrow_mut().is_ok());
            let flags = unsafe { (*child.as_ptr()).internal().state_flags.get() };

            if !flags.contains(StateFlags::SELF_DELETE) {
               Self::emit_inner_check_delete(dispatcher, &child);
               return true;
            }

            Self::emit_inner_lifecycle(
               dispatcher,
               &child,
               &LifecycleEventCtx { state: LifecycleState::Destroy { unexpected: false } },
            );
            false
         });
         //-------------------------------------------
         match input_child.try_borrow_mut() {
            Ok(mut child) => {
               let internal = child.internal_mut();
               let f = internal.state_flags.get_mut();
               f.remove(StateFlags::CHILDREN_DELETE);
               internal.set_children(children, id);
            }
            Err(e) => {
               //-----------------------
               // Children are lost so, it is attempt to inform them.
               Self::inform_lost_children(dispatcher, &children);
               //-----------------------
               log::error!(
                  "Can't borrow widget [{:?}] to process delete finalization! Children [{}] ARE LOST!\n\t{}",
                  id,
                  children.len(),
                  e
               );
               return;
            }
         };
      }
   }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

impl Dispatcher {
   /// This event should be called every program loop and actually will
   /// not perform heavy operations if they are not actually needed.
   #[cfg_attr(feature = "trace-dispatcher", tracing::instrument(level = "trace", skip_all))]
   pub fn emit_tick(&mut self, env: &mut AppEnv, event: &UpdateEvent) {
      Self::emit_inner_update(&mut self.inner, &self.root, &UpdateEventCtx { env, data: event });
      Self::emit_inner_check_delete(&mut self.inner, &self.root);
   }

   fn emit_inner_update(
      dispatcher: &mut InnerDispatcher,
      child: &Rc<RefCell<dyn IWidget>>,
      event: &UpdateEventCtx,
   ) {
      //--------------------------------------------------
      // # Safety
      // It seems it is quite safe, we just read simple copiable variables.
      // Just in case in debug mode we check availability.
      debug_assert!(child.try_borrow_mut().is_ok());
      let (state_flags, id) = unsafe {
         let internal = (*child.as_ptr()).internal();
         (internal.state_flags.get(), internal.id)
      };
      //---------------------------------
      let is_self_update = state_flags.contains(StateFlags::SELF_UPDATE);
      let is_children_update = state_flags.contains(StateFlags::CHILDREN_UPDATE);
      let is_continue = is_self_update || is_children_update;

      if !is_continue {
         return;
      }
      //--------------------------------------------------
      let mut children = match child.try_borrow_mut() {
         Ok(mut child) => {
            if is_self_update {
               let internal = child.internal_mut();
               internal.state_flags.get_mut().remove(StateFlags::SELF_UPDATE);
               child.emit_update(event);
            }

            child.internal_mut().take_children(id)
         }
         Err(e) => {
            log::error!("Can't borrow widget [{:?}] to process update event!\n\t{}", id, e);
            return;
         }
      };
      //--------------------------------------------------
      if is_children_update {
         for child in &children {
            Self::emit_inner_update(dispatcher, &child, event);
         }
      }
      //--------------------------------------------------
      match child.try_borrow_mut() {
         Ok(mut child) => {
            let internal = child.internal_mut();
            let f = internal.state_flags.get_mut();
            f.remove(StateFlags::CHILDREN_UPDATE);
            internal.set_children(children, id);
         }
         Err(e) => {
            //-----------------------
            // Children are lost so, it is attempt to inform them.
            Self::inform_lost_children(dispatcher, &children);
            //-----------------------
            log::error!(
               "Can't borrow widget [{:?}] to process update finalization! Children [{}] ARE LOST!\n\t{}",
               id,
               children.len(),
               e
            );
            return;
         }
      };
   }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

impl Dispatcher {
   #[cfg_attr(feature = "trace-dispatcher", tracing::instrument(level = "trace", skip_all))]
   pub fn emit_draw(&mut self, env: &mut AppEnv, canvas: &mut Canvas, force: bool) {
      let ev = DrawEventCtx { env, region: None };
      if !force {
         Self::emit_inner_draw(&mut self.inner, &self.root, canvas, &ev);
      } else {
         Self::emit_inner_draw_full(&mut self.inner, &self.root, canvas, &ev);
      }
   }

   fn emit_inner_draw(
      dispatcher: &mut InnerDispatcher,
      child: &Rc<RefCell<dyn IWidget>>,
      canvas: &mut Canvas,
      event: &DrawEventCtx,
   ) {
      //--------------------------------------------------
      // # Safety
      // It seems it is quite safe, we just read simple copiable variables.
      // Just in case in debug mode we check availability.
      debug_assert!(child.try_borrow_mut().is_ok());
      let (state_flags, id) = unsafe {
         let internal = (*child.as_ptr()).internal();
         (internal.state_flags.get(), internal.id)
      };
      //---------------------------------
      let is_self_draw = state_flags.contains(StateFlags::SELF_DRAW);
      let is_children_draw = state_flags.contains(StateFlags::CHILDREN_DRAW);
      let is_visible = state_flags.contains(StateFlags::IS_VISIBLE);

      if !(is_visible && (is_self_draw || is_children_draw)) {
         return;
      }
      //--------------------------------------------------
      if is_self_draw {
         Self::emit_inner_draw_full(dispatcher, &child, canvas, event);
         return;
      }
      //--------------------------------------------------
      let (children, regions) = match child.try_borrow_mut() {
         Ok(mut child) => {
            let internal = child.internal_mut();
            (internal.take_children(id), internal.take_regions(id))
         }
         Err(e) => {
            log::error!("Can't borrow widget [{:?}] to process draw event!\n\t{}", id, e);
            return;
         }
      };
      //--------------------------------------------------
      if is_children_draw {
         //---------------------------------
         // # Safety
         // It seems it is quite safe, we just read simple copiable variables.
         // Just in case in debug mode we check availability.
         // debug_assert!(child.try_borrow_mut().is_ok());
         // let (state_flags, id) = unsafe {
         //    let internal = (*child.as_ptr()).internal();
         //    (internal.state_flags.get(), internal.id)
         // };
         //
         // if state_flags.contains(StateFlags::IS_TRANSPARENT) {
         //    unimplemented!()
         // }
         //---------------------------------
         for child in &children {
            Self::emit_inner_draw(dispatcher, &child, canvas, event);
         }
      }
      //--------------------------------------------------
      match child.try_borrow_mut() {
         Ok(mut child) => {
            let internal = child.internal_mut();
            let f = internal.state_flags.get_mut();
            f.remove(StateFlags::CHILDREN_DRAW);
            internal.set_children(children, id);
            internal.set_regions(regions, id, true);
         }
         Err(e) => {
            //-----------------------
            // Children are lost so, it is attempt to inform them.
            Self::inform_lost_children(dispatcher, &children);
            //-----------------------
            log::error!(
               "Can't borrow widget [{:?}] to process draw finalization! Children [{}] ARE LOST!\n\t{}",
               id,
               children.len(),
               e
            );
            return;
         }
      };
   }

   fn emit_inner_draw_full(
      dispatcher: &mut InnerDispatcher,
      inout_child: &Rc<RefCell<dyn IWidget>>,
      canvas: &mut Canvas,
      event: &DrawEventCtx,
   ) {
      //--------------------------------------------------
      // # Safety
      // It seems it is quite safe, we just read simple copiable variables.
      // Just in case in debug mode we check availability.
      debug_assert!(inout_child.try_borrow_mut().is_ok());
      let (state_flags, id) = unsafe {
         let internal = (*inout_child.as_ptr()).internal();
         (internal.state_flags.get(), internal.id)
      };
      //---------------------------------
      let is_visible = state_flags.contains(StateFlags::IS_VISIBLE);
      if !is_visible {
         return;
      }
      //--------------------------------------------------
      let children = match inout_child.try_borrow_mut() {
         Ok(mut child) => {
            let internal = child.internal_mut();
            internal.state_flags.get_mut().remove(StateFlags::SELF_DRAW);

            // TODO draw debug bounds frame
            child.emit_draw(canvas, event);

            let internal = child.internal_mut();
            internal.take_children(id)
         }
         Err(e) => {
            log::error!("Can't borrow widget [{:?}] to process draw event!\n\t{}", id, e);
            return;
         }
      };
      //--------------------------------------------------
      for child in &children {
         Self::emit_inner_draw_full(dispatcher, &child, canvas, event);
      }
      //--------------------------------------------------
      match inout_child.try_borrow_mut() {
         Ok(mut child) => {
            let internal = child.internal_mut();
            let f = internal.state_flags.get_mut();
            f.remove(StateFlags::CHILDREN_DRAW);
            internal.set_children(children, id);
         }
         Err(e) => {
            //-----------------------
            // Children are lost so, it is attempt to inform them.
            Self::inform_lost_children(dispatcher, &children);
            //-----------------------
            log::error!(
               "Can't borrow widget [{:?}] to process draw finalization! Children [{}] ARE LOST!\n\t{}",
               id,
               children.len(),
               e
            );
            return;
         }
      };
   }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

impl Dispatcher {
   #[cfg_attr(feature = "trace-dispatcher", tracing::instrument(level = "trace", skip_all))]
   pub fn emit_mouse_move(&mut self, event: &MouseMoveEventCtx) -> bool {
      //--------------------------------------------------
      let accepted = if let Some(wmo) = &self.inner.widget_mouse_over {
         if let Some(w) = wmo.upgrade() {
            let mut widget = w.borrow_mut();
            let is_inside =
               widget.internal().geometry.rect().is_inside(event.input.x, event.input.y);

            if !is_inside {
               widget.emit_mouse_leave();
               self.inner.widget_mouse_over = None;
               false
            } else {
               widget.emit_mouse_move(event)
            }
         } else {
            false
         }
      } else {
         false
      };
      //--------------------------------------------------
      if !accepted {
         Self::emit_inner_mouse_move(&mut self.inner, &self.root, event)
      } else {
         false
      }
      //--------------------------------------------------
   }

   fn emit_inner_mouse_move(
      dispatcher: &mut InnerDispatcher,
      input_child: &Rc<RefCell<dyn IWidget>>,
      event: &MouseMoveEventCtx,
   ) -> bool {
      //--------------------------------------------------
      // # Safety
      // It seems it is quite safe, we just read simple copiable variables.
      // Just in case in debug mode we check availability.
      debug_assert!(input_child.try_borrow_mut().is_ok());
      let (id, state_flags, rect) = unsafe {
         let internal = (*input_child.as_ptr()).internal();
         (internal.id, internal.state_flags.get(), internal.geometry.rect())
      };
      //---------------------------------
      let is_enabled = state_flags.contains(StateFlags::IS_ENABLED);
      let is_inside = rect.is_inside(event.input.x, event.input.y);

      if !is_enabled || !is_inside {
         return false;
      }

      // TODO optimization, probably a parameter "mouse tracking" maybe as draw/update way.
      // TODO Enter/Leave events.
      //--------------------------------------------------
      let children = match input_child.try_borrow_mut() {
         Ok(mut child) => child.internal_mut().take_children(id),
         Err(e) => {
            log::error!("Can't borrow widget [{:?}] to process mouse move event!\n\t{}", id, e);
            return false;
         }
      };
      //--------------------------------------------------
      let mut accepted = false;
      for child in &children {
         if Self::emit_inner_mouse_move(dispatcher, &child, event) {
            accepted = true;
            break;
         }
      }
      //--------------------------------------------------
      match input_child.try_borrow_mut() {
         Ok(mut child) => {
            child.internal_mut().set_children(children, id);
            child.emit_mouse_enter();
            dispatcher.widget_mouse_over = Some(Rc::downgrade(input_child));
            if !accepted && child.emit_mouse_move(event) {
               accepted = true;
            }
         }
         Err(e) => {
            //-----------------------
            // Children are lost so, it is attempt to inform them.
            Self::inform_lost_children(dispatcher, &children);
            //-----------------------
            log::error!(
               "Can't borrow widget [{:?}] to process mouse move finalization! Children [{}] ARE LOST!\n\t{}",
               id,
               children.len(),
               e
            );
         }
      };

      accepted
   }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

impl Dispatcher {
   #[cfg_attr(feature = "trace-dispatcher", tracing::instrument(level = "trace", skip_all))]
   pub fn emit_mouse_button(&mut self, event: &MouseButtonsEventCtx) -> bool {
      Self::emit_inner_mouse_button(&mut self.inner, &self.root, event)
   }

   fn emit_inner_mouse_button(
      dispatcher: &mut InnerDispatcher,
      child: &Rc<RefCell<dyn IWidget>>,
      event: &MouseButtonsEventCtx,
   ) -> bool {
      //--------------------------------------------------
      // # Safety
      // It seems it is quite safe, we just read simple copiable variables.
      // Just in case in debug mode we check availability.
      debug_assert!(child.try_borrow_mut().is_ok());
      let (id, flow, rect) = unsafe {
         let internal = (*child.as_ptr()).internal();
         (internal.id, internal.state_flags.get(), internal.geometry.rect())
      };
      //---------------------------------
      let is_enabled = flow.contains(StateFlags::IS_ENABLED);
      let is_inside = rect.is_inside(event.input.x, event.input.y);

      if !is_enabled {
         return is_inside;
      }
      //--------------------------------------------------
      let children = match child.try_borrow_mut() {
         Ok(mut child) => child.internal_mut().take_children(id),
         Err(e) => {
            log::error!("Can't borrow widget [{:?}] to process mouse button event!\n\t{}", id, e);
            return false;
         }
      };
      //--------------------------------------------------
      let mut accepted = false;
      for child in &children {
         if Self::emit_inner_mouse_button(dispatcher, &child, event) {
            accepted = true;
            break;
         }
      }
      //--------------------------------------------------
      match child.try_borrow_mut() {
         Ok(mut child) => {
            child.internal_mut().set_children(children, id);
            if !accepted && child.emit_mouse_button(event) {
               accepted = true;
            }
         }
         Err(e) => {
            //-----------------------
            // Children are lost so, it is attempt to inform them.
            Self::inform_lost_children(dispatcher, &children);
            //-----------------------
            log::error!(
               "Can't borrow widget [{:?}] to process mouse button finalization! Children [{}] ARE LOST!\n\t{}",
               id,
               children.len(),
               e
            );
         }
      };

      accepted
   }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

impl Dispatcher {
   #[cfg_attr(feature = "trace-dispatcher", tracing::instrument(level = "trace", skip_all))]
   pub fn emit_mouse_wheel(&mut self, event: &MouseWheelEventCtx) -> bool {
      Self::emit_inner_mouse_wheel(&mut self.inner, &self.root, event)
   }

   fn emit_inner_mouse_wheel(
      dispatcher: &mut InnerDispatcher,
      child: &Rc<RefCell<dyn IWidget>>,
      event: &MouseWheelEventCtx,
   ) -> bool {
      //--------------------------------------------------
      // # Safety
      // It seems it is quite safe, we just read simple copiable variables.
      // Just in case in debug mode we check availability.
      debug_assert!(child.try_borrow_mut().is_ok());
      let (id, flow, rect) = unsafe {
         let internal = (*child.as_ptr()).internal();
         (internal.id, internal.state_flags.get(), internal.geometry.rect())
      };
      //---------------------------------
      let is_enabled = flow.contains(StateFlags::IS_ENABLED);
      let is_inside = rect.is_inside(event.input.x, event.input.y);

      if !is_enabled {
         return is_inside;
      }
      //--------------------------------------------------
      let children = match child.try_borrow_mut() {
         Ok(mut child) => child.internal_mut().take_children(id),
         Err(e) => {
            log::error!("Can't borrow widget [{:?}] to process mouse wheel event!\n\t{}", id, e);
            return false;
         }
      };
      //--------------------------------------------------
      let mut accepted = false;
      for child in &children {
         if Self::emit_inner_mouse_wheel(dispatcher, &child, event) {
            accepted = true;
            break;
         }
      }
      //--------------------------------------------------
      match child.try_borrow_mut() {
         Ok(mut child) => {
            child.internal_mut().set_children(children, id);
            if !accepted && child.emit_mouse_wheel(event) {
               accepted = true;
            }
         }
         Err(e) => {
            //-----------------------
            // Children are lost so, it is attempt to inform them.
            Self::inform_lost_children(dispatcher, &children);
            //-----------------------
            log::error!(
               "Can't borrow widget [{:?}] to process mouse wheel finalization! Children [{}] ARE LOST!\n\t{}",
               id,
               children.len(),
               e
            );
         }
      };

      accepted
   }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

impl Dispatcher {
   #[cfg_attr(feature = "trace-dispatcher", tracing::instrument(level = "trace", skip_all))]
   pub fn emit_keyboard(&mut self, event: &KeyboardEventCtx) -> bool {
      Self::emit_inner_keyboard(&mut self.inner, &self.root, event)
   }

   pub fn emit_inner_keyboard(
      dispatcher: &mut InnerDispatcher,
      child: &Rc<RefCell<dyn IWidget>>,
      event: &KeyboardEventCtx,
   ) -> bool {
      //--------------------------------------------------
      // # Safety
      // It seems it is quite safe, we just read simple copiable variables.
      // Just in case in debug mode we check availability.
      debug_assert!(child.try_borrow_mut().is_ok());
      let (id, flow) = unsafe {
         let internal = (*child.as_ptr()).internal();
         (internal.id, internal.state_flags.get())
      };
      //---------------------------------
      let is_enabled = flow.contains(StateFlags::IS_ENABLED);
      if !is_enabled {
         return false;
      }
      //--------------------------------------------------
      let children = match child.try_borrow_mut() {
         Ok(mut child) => child.internal_mut().take_children(id),
         Err(e) => {
            log::error!("Can't borrow widget [{:?}] to process mouse wheel event!\n\t{}", id, e);
            return false;
         }
      };
      //--------------------------------------------------
      let mut accepted = false;
      for child in &children {
         if Self::emit_inner_keyboard(dispatcher, &child, event) {
            accepted = true;
            break;
         }
      }
      //--------------------------------------------------
      match child.try_borrow_mut() {
         Ok(mut child) => {
            child.internal_mut().set_children(children, id);
            if !accepted && child.emit_keyboard(event) {
               accepted = true;
            }
         }
         Err(e) => {
            //-----------------------
            // Children are lost so, it is attempt to inform them.
            Self::inform_lost_children(dispatcher, &children);
            //-----------------------
            log::error!(
               "Can't borrow widget [{:?}] to process mouse wheel finalization! Children [{}] ARE LOST!\n\t{}",
               id,
               children.len(),
               e
            );
         }
      };

      accepted
   }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

impl Dispatcher {
   #[cfg_attr(feature = "trace-dispatcher", tracing::instrument(level = "trace", skip_all))]
   fn inform_lost_children(
      dispatcher: &mut InnerDispatcher,
      children: &[Rc<RefCell<dyn IWidget>>],
   ) {
      for child in children {
         Self::emit_inner_lifecycle(
            dispatcher,
            &child,
            &LifecycleEventCtx { state: LifecycleState::Destroy { unexpected: true } },
         );
      }
   }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
   use super::*;

   #[test]
   fn sizes() {
      dbg!(std::mem::size_of::<Dispatcher>());
   }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
