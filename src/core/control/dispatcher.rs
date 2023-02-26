use super::*;

use crate::core::control::runtime::Runtime;
use crate::core::events::{
   DrawEventCtx, KeyboardEventCtx, LayoutEventCtx, LifecycleEventCtx, MouseButtonsEventCtx,
   MouseMoveEventCtx, MouseWheelEventCtx, UpdateEventCtx,
};
use crate::core::{IWidget, Widget};
use sim_draw::Canvas;
use std::cell::RefCell;
use std::rc::Rc;

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct InnerDispatcher {
   runtime: Runtime,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct Dispatcher {
   inner: InnerDispatcher,
   root: Rc<RefCell<dyn IWidget>>,
}

impl Dispatcher {
   #[inline]
   pub fn new(root: Option<Rc<RefCell<dyn IWidget>>>) -> Self {
      let mut out = Self {
         inner: InnerDispatcher { runtime: Runtime::new() },
         root: root.unwrap_or_else(|| Widget::new(|_| (), |_| ())),
      };

      out.set_runtime_to_widget();
      out
   }

   #[inline]
   pub fn reinit(&mut self, root: Rc<RefCell<dyn IWidget>>) {
      self.root = root;
      self.set_runtime_to_widget();
   }

   #[inline]
   pub fn widget(&self) -> &Rc<RefCell<dyn IWidget>> {
      &self.root
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
   pub fn emit_lifecycle(&mut self, event: &LifecycleEventCtx) {
      Self::emit_inner_lifecycle(&mut self.inner, &self.root, event);
   }

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
               "Can't borrow widget [{:?}] to process lifecycle finalization! \
               Children [{}] ARE LOST!\n\t{}",
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
   pub fn emit_update(&mut self, event: &UpdateEventCtx) {
      Self::emit_inner_update(&mut self.inner, &self.root, event);
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
      let (flow, id) = unsafe {
         let internal = (*child.as_ptr()).internal();
         (internal.control_flow.get(), internal.id)
      };
      //---------------------------------
      #[cfg(debug_assertions)]
      {
         if flow.contains(ControlFlow::SELF_DELETE) {
            log::error!(
               "Not expected flag [{:?}] in [{:?}] with all flags: [{:?}]",
               ControlFlow::SELF_DELETE,
               id,
               flow
            )
         }
      }
      //---------------------------------
      let is_self_delete = flow.contains(ControlFlow::SELF_DELETE);
      let is_self_update = flow.contains(ControlFlow::SELF_UPDATE);
      let is_children_delete = flow.contains(ControlFlow::CHILDREN_DELETE);
      let is_children_update = flow.contains(ControlFlow::CHILDREN_UPDATE);
      let is_continue =
         is_self_delete || is_self_update || is_children_delete || is_children_update;

      if !is_continue {
         return;
      }
      //--------------------------------------------------
      // SELF UPDATE AND CHILDREN

      let mut children = match child.try_borrow_mut() {
         Ok(mut child) => {
            if is_self_update {
               let internal = child.internal_mut();
               // Only root widget can have self delete flag and it is should be
               // processed by this function caller, so if it is not deletes we
               // just assume that it is not necessary so, the flag is cleared.
               internal.control_flow.get_mut().remove(ControlFlow::SELF_DELETE);
               internal.control_flow.get_mut().remove(ControlFlow::SELF_UPDATE);
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
      // DELETE

      if is_children_delete {
         children.retain(|child| {
            // # Safety
            // It seems it is quite safe, we just read simple copiable variables.
            // Just in case in debug mode we check availability.
            debug_assert!(child.try_borrow_mut().is_ok());
            let flow = unsafe { (*child.as_ptr()).internal().control_flow.get() };
            //---------------------------------
            if !flow.contains(ControlFlow::SELF_DELETE) {
               return true;
            }
            //---------------------------------
            Self::emit_inner_lifecycle(
               dispatcher,
               &child,
               &LifecycleEventCtx::Delete { unexpected: false },
            );
            //---------------------------------
            false
         });
      }
      //--------------------------------------------------
      // UPDATE CHILDREN

      if is_children_update {
         for child in &children {
            Self::emit_inner_update(dispatcher, &child, event);
         }
      }
      //--------------------------------------------------
      // FINALIZE

      match child.try_borrow_mut() {
         Ok(mut child) => {
            let internal = child.internal_mut();
            let f = internal.control_flow.get_mut();
            f.remove(ControlFlow::SELF_UPDATE);
            f.remove(ControlFlow::SELF_DELETE);
            f.remove(ControlFlow::CHILDREN_UPDATE);
            f.remove(ControlFlow::CHILDREN_DELETE);
            internal.set_children(children, id);
         }
         Err(e) => {
            //-----------------------
            // Children are lost so, it is attempt to inform them.
            for child in &children {
               Self::emit_inner_lifecycle(
                  dispatcher,
                  &child,
                  &LifecycleEventCtx::Delete { unexpected: true },
               );
            }
            //-----------------------
            log::error!(
               "Can't borrow widget [{:?}] to process update finalization! \
               Children [{}] ARE LOST!\n\t{}",
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
   pub fn emit_draw(&mut self, canvas: &mut Canvas, force: bool) {
      Self::emit_inner_draw(&mut self.inner, &self.root, canvas, &DrawEventCtx {}, force);
   }

   fn emit_inner_draw(
      dispatcher: &mut InnerDispatcher,
      child: &Rc<RefCell<dyn IWidget>>,
      canvas: &mut Canvas,
      event: &DrawEventCtx,
      force: bool,
   ) {
      //--------------------------------------------------
      // # Safety
      // It seems it is quite safe, we just read simple copiable variables.
      // Just in case in debug mode we check availability.
      debug_assert!(child.try_borrow_mut().is_ok());
      let (flow, id) = unsafe {
         let internal = (*child.as_ptr()).internal();
         (internal.control_flow.get(), internal.id)
      };
      //---------------------------------
      let is_self_draw = flow.contains(ControlFlow::SELF_DRAW);
      let is_children_draw = flow.contains(ControlFlow::CHILDREN_DRAW);
      let is_visible = flow.contains(ControlFlow::IS_VISIBLE);
      let is_full_redraw = is_self_draw || force;

      if !(is_visible && (is_self_draw || is_children_draw)) {
         return;
      }
      //--------------------------------------------------
      let (children, regions) = match child.try_borrow_mut() {
         Ok(mut child) => {
            let internal = child.internal_mut();

            if is_full_redraw {
               internal.control_flow.get_mut().remove(ControlFlow::SELF_DRAW);
               child.emit_draw(canvas, event);
            }

            let internal = child.internal_mut();
            (internal.take_children(id), internal.take_regions(id))
         }
         Err(e) => {
            log::error!("Can't borrow widget [{:?}] to process draw event!\n\t{}", id, e);
            return;
         }
      };
      //--------------------------------------------------
      for child in &children {
         Self::emit_inner_draw(dispatcher, &child, canvas, event, force);
      }
      //--------------------------------------------------
      match child.try_borrow_mut() {
         Ok(mut child) => {
            let internal = child.internal_mut();
            let f = internal.control_flow.get_mut();
            f.remove(ControlFlow::SELF_DRAW);
            f.remove(ControlFlow::CHILDREN_DRAW);
            internal.set_children(children, id);
            internal.set_regions(regions, id, true);
         }
         Err(e) => {
            //-----------------------
            // Children are lost so, it is attempt to inform them.
            for child in &children {
               Self::emit_inner_lifecycle(
                  dispatcher,
                  &child,
                  &LifecycleEventCtx::Delete { unexpected: true },
               );
            }
            //-----------------------
            log::error!(
               "Can't borrow widget [{:?}] to process draw finalization! \
               Children [{}] ARE LOST!\n\t{}",
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
   pub fn emit_mouse_move(&mut self, event: &MouseMoveEventCtx) -> bool {
      Self::emit_inner_mouse_move(&mut self.inner, &self.root, event)
   }

   fn emit_inner_mouse_move(
      dispatcher: &mut InnerDispatcher,
      child: &Rc<RefCell<dyn IWidget>>,
      event: &MouseMoveEventCtx,
   ) -> bool {
      //--------------------------------------------------
      // # Safety
      // It seems it is quite safe, we just read simple copiable variables.
      // Just in case in debug mode we check availability.
      debug_assert!(child.try_borrow_mut().is_ok());
      let (id, flow, rect) = unsafe {
         let internal = (*child.as_ptr()).internal();
         (internal.id, internal.control_flow.get(), internal.geometry.rect())
      };
      //---------------------------------
      let is_enabled = flow.contains(ControlFlow::IS_ENABLED);
      let is_inside = rect.is_inside(event.input.x, event.input.y);

      if !is_enabled {
         return is_inside;
      }

      // TODO optimization, probably a parameter "mouse tracking" may as draw/update way.
      //--------------------------------------------------
      let children = match child.try_borrow_mut() {
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
      match child.try_borrow_mut() {
         Ok(mut child) => {
            child.internal_mut().set_children(children, id);
            if !accepted && child.emit_mouse_move(event) {
               accepted = true;
            }
         }
         Err(e) => {
            //-----------------------
            // Children are lost so, it is attempt to inform them.
            for child in &children {
               Self::emit_inner_lifecycle(
                  dispatcher,
                  &child,
                  &LifecycleEventCtx::Delete { unexpected: true },
               );
            }
            //-----------------------
            log::error!(
               "Can't borrow widget [{:?}] to process mouse move finalization! \
               Children [{}] ARE LOST!\n\t{}",
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
         (internal.id, internal.control_flow.get(), internal.geometry.rect())
      };
      //---------------------------------
      let is_enabled = flow.contains(ControlFlow::IS_ENABLED);
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
            for child in &children {
               Self::emit_inner_lifecycle(
                  dispatcher,
                  &child,
                  &LifecycleEventCtx::Delete { unexpected: true },
               );
            }
            //-----------------------
            log::error!(
               "Can't borrow widget [{:?}] to process mouse button finalization! \
                Children [{}] ARE LOST!\n\t{}",
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
         (internal.id, internal.control_flow.get(), internal.geometry.rect())
      };
      //---------------------------------
      let is_enabled = flow.contains(ControlFlow::IS_ENABLED);
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
            for child in &children {
               Self::emit_inner_lifecycle(
                  dispatcher,
                  &child,
                  &LifecycleEventCtx::Delete { unexpected: true },
               );
            }
            //-----------------------
            log::error!(
               "Can't borrow widget [{:?}] to process mouse wheel finalization! \
               Children [{}] ARE LOST!\n\t{}",
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
         (internal.id, internal.control_flow.get())
      };
      //---------------------------------
      let is_enabled = flow.contains(ControlFlow::IS_ENABLED);
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
            for child in &children {
               Self::emit_inner_lifecycle(
                  dispatcher,
                  &child,
                  &LifecycleEventCtx::Delete { unexpected: true },
               );
            }
            //-----------------------
            log::error!(
               "Can't borrow widget [{:?}] to process mouse wheel finalization! \
               Children [{}] ARE LOST!\n\t{}",
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
