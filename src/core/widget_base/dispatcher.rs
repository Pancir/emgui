use super::*;

use crate::core::events::{
   DrawEventCtx, KeyboardEventCtx, LayoutEventCtx, LifecycleEventCtx, LifecycleState,
   MouseButtonsEventCtx, MouseMoveEventCtx, MouseWheelEventCtx, UpdateEventCtx,
};
use crate::core::widget_base::runtime::Runtime;
use crate::core::{AppEnv, IWidget};
use crate::widgets::Widget;
use sim_draw::Canvas;
use sim_input::mouse::MouseState;
use sim_run::UpdateEvent;
use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct InnerDispatcher {
   widget_mouse_over: Option<Weak<RefCell<dyn IWidget>>>,
   widget_mouse_button: Option<Weak<RefCell<dyn IWidget>>>,
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
         inner: InnerDispatcher {
            runtime: Runtime::new(),
            widget_mouse_over: None,
            widget_mouse_button: None,
         },
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

   #[inline]
   pub fn set_tool_type_duration(&self, duration: Duration) {
      self.inner.runtime.set_tool_type_time(duration);
   }

   #[inline]
   pub fn tool_type_duration(&self) -> Duration {
      self.inner.runtime.tool_type_time()
   }

   #[inline]
   pub fn set_double_click_duration(&self, duration: Duration) {
      self.inner.runtime.set_double_click_time(duration);
   }

   #[inline]
   pub fn double_click_duration(&self) -> Duration {
      self.inner.runtime.double_click_time()
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
            let mut internal = bor.base_mut();
            internal.runtime = Some(runtime.clone());

            let children = internal.take_children(internal.id);
            for child in &children {
               Self::set_runtime_to_widget_inner(runtime.clone(), &child);
            }
            internal.set_children(children, internal.id)
         }
         Err(e) => panic!("{:?}", e),
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
         let internal = (*child.as_ptr()).base();
         internal.id
      };
      //---------------------------------
      let children = match child.try_borrow_mut() {
         Ok(mut child) => child.base_mut().take_children(id),
         Err(e) => {
            log::error!("Can't borrow widget [{:?}] to process lifecycle event!\n\t{:?}", id, e);
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
            child.base_mut().set_children(children, id);
            child.emit_lifecycle(event);
         }
         Err(e) => {
            log::error!(
               "Can't borrow widget [{:?}] to process lifecycle finalization! Children [{}] ARE LOST!\n\t{:?}",
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
            child.base_mut().take_children(id)
         }
         Err(e) => {
            panic!("{:?}", e)
         }
      };
      //--------------------------------------------------
      if !children.is_empty() {
         for child in &children {
            Self::emit_inner_layout(dispatcher, &child, event);
         }
      }
      //--------------------------------------------------
      let mut bor = child.try_borrow_mut().unwrap_or_else(|e| panic!("{:?}", e));
      let id = bor.id();
      bor.base_mut().set_children(children, id);
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
         let internal = (*input_child.as_ptr()).base();
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
            Ok(mut child) => child.base_mut().take_children(id),
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
            let flags = unsafe { (*child.as_ptr()).base().state_flags.get() };

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
               let internal = child.base_mut();
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
                  "Can't borrow widget [{:?}] to process delete finalization! Children [{}] ARE LOST!\n\t{:?}",
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
         let internal = (*child.as_ptr()).base();
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
      let children = match child.try_borrow_mut() {
         Ok(mut child) => {
            if is_self_update {
               let internal = child.base_mut();
               internal.state_flags.get_mut().remove(StateFlags::SELF_UPDATE);
               child.emit_update(event);
            }
            child.base_mut().take_children(id)
         }
         Err(e) => {
            log::error!("Can't borrow widget [{:?}] to process update event!\n\t{:?}", id, e);
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
            let internal = child.base_mut();
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
               "Can't borrow widget [{:?}] to process update finalization! Children [{}] ARE LOST!\n\t{:?}",
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

   /// There are several situations.
   ///
   /// * Full redraw including all children
   /// * Redraw without redraw children. Probably masked (clipped) draw.
   ///   - What if children have transparent pixels? It seems we need to redraw the whole tree part.
   /// * Redraw children without transparent pixels. Actually only children redrawn.
   /// * Redraw children with transparent pixels. Probably masked + children.
   /// * Redraw animated.
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
         let internal = (*child.as_ptr()).base();
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
         // TODO it seems it is no a good idea to redraw children as well.
         // Use case: heavy children but this widget change its background on mouse enter/leave.
         Self::emit_inner_draw_full(dispatcher, &child, canvas, event);
         return;
      }
      //--------------------------------------------------
      let children = match child.try_borrow_mut() {
         Ok(mut child) => {
            let internal = child.base_mut();
            internal.take_children(id)
         }
         Err(e) => {
            log::error!("Can't borrow widget [{:?}] to process draw event!\n\t{:?}", id, e);
            return;
         }
      };
      //--------------------------------------------------
      if is_children_draw {
         //---------------------------------
         // TODO transparency processing.
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
            let internal = child.base_mut();
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
               "Can't borrow widget [{:?}] to process draw finalization! Children [{}] ARE LOST!\n\t{:?}",
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
         let internal = (*inout_child.as_ptr()).base();
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
            let internal = child.base_mut();
            internal.state_flags.get_mut().remove(StateFlags::SELF_DRAW);

            // TODO draw debug bounds frame
            child.emit_draw(canvas, event);

            let internal = child.base_mut();
            internal.take_children(id)
         }
         Err(e) => {
            log::error!("Can't borrow widget [{:?}] to process draw event!\n\t{:?}", id, e);
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
            let internal = child.base_mut();
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
               "Can't borrow widget [{:?}] to process draw finalization! Children [{}] ARE LOST!\n\t{:?}",
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
      if let Some(wmo) = &self.inner.widget_mouse_over {
         if let Some(w) = wmo.upgrade() {
            //----------------------------------
            // # Safety
            // It seems it is quite safe, we just read simple copiable variables.
            // Just in case in debug mode we check availability.
            debug_assert!(w.try_borrow_mut().is_ok());
            let (rect, mouse_btn_num, mouse_tracking) = unsafe {
               let internal = (*w.as_ptr()).base();
               (internal.geometry.rect(), internal.mouse_btn_num(), internal.has_mouse_tracking())
            };
            //----------------------------------
            let is_inside = rect.is_inside(event.input.x, event.input.y);

            if is_inside {
               if mouse_btn_num > 0 || mouse_tracking {
                  let mut widget = w.borrow_mut();
                  widget.emit_mouse_move(event);
                  return true;
               }
            } else {
               let mut widget = w.borrow_mut();

               if mouse_btn_num > 0 {
                  widget.emit_mouse_move(event);
                  return true;
               }

               let internal = widget.base_mut();

               internal.set_over(false);
               widget.emit_mouse_leave();
               self.inner.widget_mouse_over = None;
            }
         }
      }
      //--------------------------------------------------
      Self::emit_inner_mouse_move(&mut self.inner, &self.root, event)
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
         let internal = (*input_child.as_ptr()).base();
         (internal.id, internal.state_flags.get(), internal.geometry.rect())
      };
      //---------------------------------
      let is_enabled = state_flags.contains(StateFlags::IS_ENABLED);
      let is_inside = rect.is_inside(event.input.x, event.input.y);

      if !is_enabled || !is_inside {
         return false;
      }
      //--------------------------------------------------
      let children = match input_child.try_borrow_mut() {
         Ok(mut child) => child.base_mut().take_children(id),
         Err(e) => {
            log::error!("Can't borrow widget [{:?}] to process mouse move event!\n\t{:?}", id, e);
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
            let internal = child.base_mut();
            internal.set_children(children, id);
            if !accepted {
               if !internal.is_over() {
                  internal.set_over(true);
                  // child.emit_mouse_enter();
                  // this is not needed anymore so, it is free from borrowing now
                  // and the following code can be safely invoked.
                  drop(child);

                  if let Some(wmo) = &dispatcher.widget_mouse_over {
                     if let Some(w) = wmo.upgrade() {
                        let mut widget = w.borrow_mut();
                        let internal = widget.base_mut();

                        internal.set_over(false);
                        widget.emit_mouse_leave();
                     }
                  }

                  dispatcher.widget_mouse_over = Some(Rc::downgrade(input_child));

                  // now enter mouse event is needed so, we again trying to borrow.
                  match input_child.try_borrow_mut() {
                     Ok(mut child) => {
                        child.emit_mouse_enter();
                        // checking buttons pressing does not make sense in this location.
                        if child.base().has_mouse_tracking() {
                           child.emit_mouse_move(event);
                        }
                     }
                     Err(e) => {
                        log::error!(
                           "Can't borrow widget [{:?}] to process mouse enter! {:?}",
                           id,
                           e
                        );
                     }
                  }
               }
            }
            accepted = true;
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
      //--------------------------------------------------
      accepted
   }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

impl Dispatcher {
   #[cfg_attr(feature = "trace-dispatcher", tracing::instrument(level = "trace", skip_all))]
   pub fn emit_mouse_button(&mut self, event: &MouseButtonsEventCtx) -> bool {
      //--------------------------------------------------
      if let Some(wmo) = &self.inner.widget_mouse_button {
         if let Some(w) = wmo.upgrade() {
            //----------------------------------
            // # Safety
            // It seems it is quite safe, we just read simple copiable variables.
            // Just in case in debug mode we check availability.
            debug_assert!(w.try_borrow_mut().is_ok());
            let rect = unsafe {
               let internal = (*w.as_ptr()).base();
               internal.geometry.rect()
            };
            //----------------------------------
            let is_inside = rect.is_inside(event.input.x, event.input.y);
            let mut widget = w.borrow_mut();

            return match event.input.state {
               MouseState::Pressed => {
                  if is_inside {
                     widget.emit_mouse_button(event);
                     let internal = widget.base_mut();
                     internal.add_mouse_btn_num(1);
                  }
                  true
               }
               MouseState::Released => {
                  widget.emit_mouse_button(event);
                  let internal = widget.base_mut();

                  internal.add_mouse_btn_num(-1);
                  if internal.mouse_btn_num() == 0 {
                     self.inner.widget_mouse_button = None;
                  }
                  true
               }
            };
         }
      }
      //--------------------------------------------------
      Self::emit_inner_mouse_button(&mut self.inner, &self.root, event)
      //--------------------------------------------------
   }

   fn emit_inner_mouse_button(
      dispatcher: &mut InnerDispatcher,
      input_child: &Rc<RefCell<dyn IWidget>>,
      event: &MouseButtonsEventCtx,
   ) -> bool {
      //--------------------------------------------------
      // # Safety
      // It seems it is quite safe, we just read simple copiable variables.
      // Just in case in debug mode we check availability.
      debug_assert!(input_child.try_borrow_mut().is_ok());
      let (id, flow, rect) = unsafe {
         let internal = (*input_child.as_ptr()).base();
         (internal.id, internal.state_flags.get(), internal.geometry.rect())
      };
      //---------------------------------
      let is_enabled = flow.contains(StateFlags::IS_ENABLED);
      let is_inside = rect.is_inside(event.input.x, event.input.y);

      if !is_enabled || !is_inside {
         return false;
      }
      //--------------------------------------------------
      let children = match input_child.try_borrow_mut() {
         Ok(mut child) => child.base_mut().take_children(id),
         Err(e) => {
            log::error!("Can't borrow widget [{:?}] to process mouse button event!\n\t{:?}", id, e);
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
      match input_child.try_borrow_mut() {
         Ok(mut child) => {
            child.base_mut().set_children(children, id);
            if !accepted {
               debug_assert!(
                  event.input.state != MouseState::Released,
                  "expected to be processed before"
               );
               child.base_mut().add_mouse_btn_num(1);
               accepted = child.emit_mouse_button(event);
               dispatcher.widget_mouse_button = Some(Rc::downgrade(&input_child));
            }
         }
         Err(e) => {
            //-----------------------
            // Children are lost so, it is attempt to inform them.
            Self::inform_lost_children(dispatcher, &children);
            //-----------------------
            log::error!(
               "Can't borrow widget [{:?}] to process mouse button finalization! Children [{}] ARE LOST!\n\t{:?}",
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
         let internal = (*child.as_ptr()).base();
         (internal.id, internal.state_flags.get(), internal.geometry.rect())
      };
      //---------------------------------
      let is_enabled = flow.contains(StateFlags::IS_ENABLED);
      let is_inside = rect.is_inside(event.input.x, event.input.y);

      if !is_enabled || !is_inside {
         return false;
      }
      //--------------------------------------------------
      let children = match child.try_borrow_mut() {
         Ok(mut child) => child.base_mut().take_children(id),
         Err(e) => {
            log::error!("Can't borrow widget [{:?}] to process mouse wheel event!\n\t{:?}", id, e);
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
            child.base_mut().set_children(children, id);
            if !accepted {
               accepted = child.emit_mouse_wheel(event);
            }
         }
         Err(e) => {
            //-----------------------
            // Children are lost so, it is attempt to inform them.
            Self::inform_lost_children(dispatcher, &children);
            //-----------------------
            log::error!(
               "Can't borrow widget [{:?}] to process mouse wheel finalization! Children [{}] ARE LOST!\n\t{:?}",
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
         let internal = (*child.as_ptr()).base();
         (internal.id, internal.state_flags.get())
      };
      //---------------------------------
      let is_enabled = flow.contains(StateFlags::IS_ENABLED);
      if !is_enabled {
         return false;
      }
      //--------------------------------------------------
      let children = match child.try_borrow_mut() {
         Ok(mut child) => child.base_mut().take_children(id),
         Err(e) => {
            log::error!("Can't borrow widget [{:?}] to process mouse wheel event!\n\t{:?}", id, e);
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
            child.base_mut().set_children(children, id);
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
               "Can't borrow widget [{:?}] to process mouse wheel finalization! Children [{}] ARE LOST!\n\t{:?}",
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
   use crate::core::derive::Derive;
   use sim_draw::m::Rect;
   use sim_input::mouse::MouseMoveInput;
   use sim_input::{DeviceId, Modifiers};
   use std::any::Any;

   //---------------------------------------------------

   pub struct TestWidget {
      pub mouse_enter: usize,
      pub mouse_leave: usize,
   }

   impl TestWidget {
      pub fn new(rect: Rect<f32>) -> Rc<RefCell<Widget<Self>>> {
         Widget::new(
            |vt| {
               vt.on_mouse_enter = |w: &mut Widget<Self>| w.derive_mut().mouse_enter += 1;
               vt.on_mouse_leave = |w: &mut Widget<Self>| w.derive_mut().mouse_leave += 1;
               Self { mouse_enter: 0, mouse_leave: 0 }
            },
            |w| {
               w.set_rect(rect);
            },
         )
      }
   }

   impl Derive for TestWidget {
      fn as_any(&self) -> &dyn Any {
         self
      }

      fn as_any_mut(&mut self) -> &mut dyn Any {
         self
      }
   }

   //---------------------------------------------------

   fn mouse_move_ctx(x: f32, y: f32) -> MouseMoveEventCtx {
      MouseMoveEventCtx {
         input: MouseMoveInput {
            device_id: DeviceId::default(),
            modifiers: Modifiers::default(),
            x,
            y,
         },
      }
   }

   //---------------------------------------------------

   ///
   ///      +---------------------+
   ///      |     WIDGET 1        |
   ///      |                     |
   ///      |        +------------+
   ///  1   |     2  |  WIDGET 2  |
   ///  X------>  X----->         |
   ///      |        |            |
   ///  <------X  <-----X         |
   ///      |  4     |  3         |
   ///      |        +------------+
   ///      |                     |
   ///      +---------------------+
   ///
   /// 1 - move from outside into the widget(1).
   ///     a) widget(1) enter event.
   /// 2 - move from widget(1) into the widget(2).
   ///     a) widget(1) leave event.
   ///     b) widget(2) enter event.
   /// 3 - move from widget(2) back to the widget(1).
   ///     a) widget(2) leave event.
   ///     b) widget(1) enter event.
   /// 4 - move from widget(1) back to outside.
   ///     a) widget(1) leave event.
   #[test]
   fn test_mouse_move() {
      let root = TestWidget::new(Rect::new(0.0, 0.0, 200.0, 200.0));
      let child = TestWidget::new(Rect::new(50.0, 50.0, 100.0, 100.0));

      let mut dispatcher = Dispatcher::new(Some(root.clone()));
      add_child(dispatcher.widget(), child.clone());
      //----------------------
      dispatcher.emit_mouse_move(&mouse_move_ctx(-10.0, 100.0));
      assert_eq!(root.borrow_mut().derive_mut().mouse_enter, 0);
      assert_eq!(root.borrow_mut().derive_mut().mouse_leave, 0);
      assert_eq!(child.borrow_mut().derive_mut().mouse_enter, 0);
      assert_eq!(child.borrow_mut().derive_mut().mouse_leave, 0);
      //----------------------
      dispatcher.emit_mouse_move(&mouse_move_ctx(25.0, 100.0));
      assert_eq!(root.borrow_mut().derive_mut().mouse_enter, 1);
      assert_eq!(root.borrow_mut().derive_mut().mouse_leave, 0);
      assert_eq!(child.borrow_mut().derive_mut().mouse_enter, 0);
      assert_eq!(child.borrow_mut().derive_mut().mouse_leave, 0);
      //----------------------
      dispatcher.emit_mouse_move(&mouse_move_ctx(75.0, 100.0));
      assert_eq!(root.borrow_mut().derive_mut().mouse_enter, 1);
      assert_eq!(root.borrow_mut().derive_mut().mouse_leave, 1);
      assert_eq!(child.borrow_mut().derive_mut().mouse_enter, 1);
      assert_eq!(child.borrow_mut().derive_mut().mouse_leave, 0);
      //----------------------
      dispatcher.emit_mouse_move(&mouse_move_ctx(25.0, 100.0));
      assert_eq!(root.borrow_mut().derive_mut().mouse_enter, 2);
      assert_eq!(root.borrow_mut().derive_mut().mouse_leave, 1);
      assert_eq!(child.borrow_mut().derive_mut().mouse_enter, 1);
      assert_eq!(child.borrow_mut().derive_mut().mouse_leave, 1);
      //----------------------
      dispatcher.emit_mouse_move(&mouse_move_ctx(-10.0, 100.0));
      assert_eq!(root.borrow_mut().derive_mut().mouse_enter, 2);
      assert_eq!(root.borrow_mut().derive_mut().mouse_leave, 2);
      assert_eq!(child.borrow_mut().derive_mut().mouse_enter, 1);
      assert_eq!(child.borrow_mut().derive_mut().mouse_leave, 1);
      //----------------------
   }

   //---------------------------------------------------

   #[test]
   fn sizes() {
      dbg!(std::mem::size_of::<Dispatcher>());
   }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
