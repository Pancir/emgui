use super::*;
use crate::core::events::{
   DrawEventCtx, KeyboardEventCtx, LayoutEventCtx, LifecycleEventCtx, LifecycleState,
   MouseButtonsEventCtx, MouseMoveEventCtx, MouseWheelEventCtx, UpdateEventCtx,
};
use crate::core::widget_base::runtime::Runtime;
use crate::core::{AppEnv, IWidget, Painter, WidgetRefOwner};
use crate::widgets::Widget;
use sim_input::mouse::MouseState;
use sim_run::UpdateEvent;
use std::time::Duration;

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct InnerDispatcher {
   widget_mouse_over: Option<WidgetRef>,
   widget_mouse_button: Option<WidgetRef>,
   runtime: Runtime,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct Dispatcher {
   inner: InnerDispatcher,
   root: WidgetRefOwner,
   destroyed: bool,
}

impl Dispatcher {
   #[inline]
   pub fn new(root: Option<WidgetRefOwner>, runtime: Runtime) -> Self {
      let mut out = Self {
         inner: InnerDispatcher { runtime, widget_mouse_over: None, widget_mouse_button: None },
         root: root.unwrap_or_else(|| Widget::inherit(|_| (), |_| ()).to_owner()),
         destroyed: false,
      };

      out.set_runtime_to_widget();
      out
   }

   #[inline]
   pub fn reinit(&mut self, root: WidgetRefOwner) {
      if !self.destroyed {
         self.destroy();
      }
      self.root = root;
      self.set_runtime_to_widget();
   }

   #[inline]
   pub fn widget(&self) -> &WidgetRefOwner {
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
but it may lead to unexpected behavior because it can be too late, please call it yourself."
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

   fn emit_inner_destroy(dispatcher: &mut InnerDispatcher, child: &WidgetRefOwner) {
      Self::emit_inner_lifecycle(
         dispatcher,
         child,
         &LifecycleEventCtx { state: LifecycleState::Destroy { unexpected: false } },
      );
   }
}

impl Dispatcher {
   fn set_runtime_to_widget(&mut self) {
      Self::set_runtime_to_widget_inner(self.inner.runtime.clone(), &self.root);
   }

   fn set_runtime_to_widget_inner(runtime: Runtime, child: &WidgetRefOwner) {
      match child.widget_mut() {
         Ok(mut bor) => {
            let mut internal = bor.base_mut();
            let id = internal.id;

            internal.runtime = Some(runtime.clone());

            let children = internal.children_mut().take_children(id);
            for child in &children {
               Self::set_runtime_to_widget_inner(runtime.clone(), child);
            }
            internal.children_mut().return_children(children, id)
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
      child: &WidgetRefOwner,
      event: &LifecycleEventCtx,
   ) {
      //--------------------------------------------------
      let (_state_flags, id, _rect, _mouse_btn_num, _has_mouse_tracking) =
         child.data_for_dispatcher();
      //---------------------------------
      let children = match child.widget_mut() {
         Ok(mut child) => child.base_mut().children_mut().take_children(id),
         Err(e) => {
            log::error!("Can't borrow widget [{:?}] to process lifecycle event!\n\t{:?}", id, e);
            return;
         }
      };
      //--------------------------------------------------
      if !children.is_empty() {
         for child in &children {
            Self::emit_inner_lifecycle(dispatcher, child, event);
         }
      }
      //--------------------------------------------------
      match child.widget_mut() {
         Ok(mut child) => {
            child.base_mut().children_mut().return_children(children, id);
            child.on_lifecycle(event);
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
      child: &WidgetRefOwner,
      event: &LayoutEventCtx,
   ) {
      let children = match child.widget_mut() {
         Ok(mut child) => {
            child.on_layout(event);
            let base = child.base_mut();
            let id = base.id();
            base.children_mut().take_children(id)
         }
         Err(e) => {
            panic!("{:?}", e)
         }
      };
      //--------------------------------------------------
      if !children.is_empty() {
         for child in &children {
            Self::emit_inner_layout(dispatcher, child, event);
         }
      }
      //--------------------------------------------------
      let mut bor = child.widget_mut().unwrap_or_else(|e| panic!("{:?}", e));
      let base = bor.base_mut();
      let id = base.id();
      base.children_mut().return_children(children, id);
   }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

impl Dispatcher {
   /// This event check if there are widgets to delete and perform deleting.
   #[cfg_attr(feature = "trace-dispatcher", tracing::instrument(level = "trace", skip_all))]
   fn emit_inner_check_delete(dispatcher: &mut InnerDispatcher, input_child: &WidgetRefOwner) {
      //--------------------------------------------------
      let (state_flags, id, _rect, _mouse_btn_num, _has_mouse_tracking) =
         input_child.data_for_dispatcher();
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
         let mut children = match input_child.widget_mut() {
            Ok(mut child) => child.base_mut().children_mut().take_children(id),
            Err(e) => {
               log::error!("Can't borrow widget [{:?}] to process delete event!\n\t{}", id, e);
               return;
            }
         };
         //-------------------------------------------
         children.retain(|child| {
            let (flags, _id, _rect, _mouse_btn_num, _has_mouse_tracking) =
               child.data_for_dispatcher();

            if !flags.contains(StateFlags::SELF_DELETE) {
               Self::emit_inner_check_delete(dispatcher, child);
               return true;
            }

            Self::emit_inner_lifecycle(
               dispatcher,
               child,
               &LifecycleEventCtx { state: LifecycleState::Destroy { unexpected: false } },
            );
            false
         });
         //-------------------------------------------
         match input_child.widget_mut() {
            Ok(mut child) => {
               let internal = child.base_mut();
               let f = internal.state_flags.get_mut();
               f.remove(StateFlags::CHILDREN_DELETE);
               internal.children_mut().return_children(children, id);
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
      child: &WidgetRefOwner,
      event: &UpdateEventCtx,
   ) {
      //--------------------------------------------------
      let (state_flags, id, _rect, _mouse_btn_num, _has_mouse_tracking) =
         child.data_for_dispatcher();
      //---------------------------------

      let is_self_update = state_flags.contains(StateFlags::SELF_UPDATE);
      let is_children_update = state_flags.contains(StateFlags::CHILDREN_UPDATE);
      let is_continue = is_self_update || is_children_update;

      if !is_continue {
         return;
      }
      //--------------------------------------------------
      let children = match child.widget_mut() {
         Ok(mut child) => {
            if is_self_update {
               let internal = child.base_mut();
               internal.state_flags.get_mut().remove(StateFlags::SELF_UPDATE);
               child.on_update(event);
            }
            child.base_mut().children_mut().take_children(id)
         }
         Err(e) => {
            log::error!("Can't borrow widget [{:?}] to process update event!\n\t{:?}", id, e);
            return;
         }
      };
      //--------------------------------------------------
      if is_children_update {
         for child in &children {
            Self::emit_inner_update(dispatcher, child, event);
         }
      }
      //--------------------------------------------------
      match child.widget_mut() {
         Ok(mut child) => {
            let internal = child.base_mut();
            let f = internal.state_flags.get_mut();
            f.remove(StateFlags::CHILDREN_UPDATE);
            internal.children_mut().return_children(children, id);
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
         }
      };
   }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

impl Dispatcher {
   #[cfg_attr(feature = "trace-dispatcher", tracing::instrument(level = "trace", skip_all))]
   pub fn emit_draw(&mut self, env: &mut AppEnv, canvas: &mut Painter, force: bool) {
      let ev = DrawEventCtx { env, region: None, abs_time: Duration::new(0, 0) };
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
      child: &WidgetRefOwner,
      canvas: &mut Painter,
      event: &DrawEventCtx,
   ) {
      //--------------------------------------------------
      let (state_flags, id, _rect, _mouse_btn_num, _has_mouse_tracking) =
         child.data_for_dispatcher();
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
         Self::emit_inner_draw_full(dispatcher, child, canvas, event);
         return;
      }
      //--------------------------------------------------
      let children = match child.widget_mut() {
         Ok(mut child) => {
            let internal = child.base_mut();
            internal.children_mut().take_children(id)
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
         // It seems it is quite safe, we just read simple copyable variables.
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
            Self::emit_inner_draw(dispatcher, child, canvas, event);
         }
      }
      //--------------------------------------------------
      match child.widget_mut() {
         Ok(mut child) => {
            let internal = child.base_mut();
            let f = internal.state_flags.get_mut();
            f.remove(StateFlags::CHILDREN_DRAW);
            internal.children_mut().return_children(children, id);
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
         }
      };
   }

   fn emit_inner_draw_full(
      dispatcher: &mut InnerDispatcher,
      input_child: &WidgetRefOwner,
      canvas: &mut Painter,
      event: &DrawEventCtx,
   ) {
      //--------------------------------------------------
      let (state_flags, id, _rect, _mouse_btn_num, _has_mouse_tracking) =
         input_child.data_for_dispatcher();
      //---------------------------------
      let is_visible = state_flags.contains(StateFlags::IS_VISIBLE);
      if !is_visible {
         return;
      }
      //--------------------------------------------------
      let children = match input_child.widget_mut() {
         Ok(mut child) => {
            let internal = child.base_mut();
            internal.state_flags.get_mut().remove(StateFlags::SELF_DRAW);

            // TODO draw debug bounds frame
            child.on_draw(canvas, event);

            let internal = child.base_mut();
            internal.children_mut().take_children(id)
         }
         Err(e) => {
            log::error!("Can't borrow widget [{:?}] to process draw event!\n\t{:?}", id, e);
            return;
         }
      };
      //--------------------------------------------------
      for child in &children {
         Self::emit_inner_draw_full(dispatcher, child, canvas, event);
      }
      //--------------------------------------------------
      match input_child.widget_mut() {
         Ok(mut child) => {
            let internal = child.base_mut();
            let f = internal.state_flags.get_mut();
            f.remove(StateFlags::CHILDREN_DRAW);
            internal.children_mut().return_children(children, id);
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
            let (_state_flags, _id, rect, mouse_btn_num, mouse_tracking) = w.data_for_dispatcher();
            //----------------------------------
            let is_inside = rect.is_inside(event.input.x, event.input.y);

            if is_inside {
               if mouse_btn_num > 0 || mouse_tracking {
                  let mut widget = w.widget_mut().unwrap();
                  widget.on_mouse_move(event);
                  return true;
               }
            } else {
               let mut widget = w.widget_mut().unwrap();

               if mouse_btn_num > 0 {
                  widget.on_mouse_move(event);
                  return true;
               }

               let internal = widget.base_mut();

               internal.set_over(false);
               widget.on_mouse_cross(false);
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
      input_child: &WidgetRefOwner,
      event: &MouseMoveEventCtx,
   ) -> bool {
      //--------------------------------------------------
      let (state_flags, id, rect, _mouse_btn_num, _has_mouse_tracking) =
         input_child.data_for_dispatcher();
      //---------------------------------
      let is_enabled = state_flags.contains(StateFlags::IS_ENABLED);
      let is_inside = rect.is_inside(event.input.x, event.input.y);

      if !is_enabled || !is_inside {
         return false;
      }
      //--------------------------------------------------
      let children = match input_child.widget_mut() {
         Ok(mut child) => child.base_mut().children_mut().take_children(id),
         Err(e) => {
            log::error!("Can't borrow widget [{:?}] to process mouse move event!\n\t{:?}", id, e);
            return false;
         }
      };
      //--------------------------------------------------
      let mut accepted = false;
      for child in &children {
         if Self::emit_inner_mouse_move(dispatcher, child, event) {
            accepted = true;
            break;
         }
      }
      //--------------------------------------------------
      match input_child.widget_mut() {
         Ok(mut child) => {
            let internal = child.base_mut();
            internal.children_mut().return_children(children, id);
            if !accepted && !internal.is_over() {
               internal.set_over(true);
               // child.emit_mouse_enter();
               // this is not needed anymore so, it is free from borrowing now
               // and the following code can be safely invoked.
               drop(child);

               if let Some(wmo) = &dispatcher.widget_mouse_over {
                  if let Some(w) = wmo.upgrade() {
                     let mut widget = w.widget_mut().unwrap();
                     let internal = widget.base_mut();

                     internal.set_over(false);
                     widget.on_mouse_cross(false);
                  }
               }

               dispatcher.widget_mouse_over = Some(input_child.as_ref());

               // now enter mouse event is needed so, we again trying to borrow.
               match input_child.widget_mut() {
                  Ok(mut child) => {
                     child.on_mouse_cross(true);
                     // checking buttons pressing does not make sense in this location.
                     if child.base().has_mouse_tracking() {
                        child.on_mouse_move(event);
                     }
                  }
                  Err(e) => {
                     log::error!("Can't borrow widget [{:?}] to process mouse enter! {:?}", id, e);
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
            let (_state_flags, _id, rect, _mouse_btn_num, _has_mouse_tracking) =
               w.data_for_dispatcher();
            //----------------------------------
            let is_inside = rect.is_inside(event.input.x, event.input.y);
            let mut widget = w.widget_mut().unwrap();

            return match event.input.state {
               MouseState::Pressed => {
                  if is_inside {
                     widget.on_mouse_button(event);
                     let internal = widget.base_mut();
                     internal.add_mouse_btn_num(1);
                  }
                  true
               }
               MouseState::Released => {
                  widget.on_mouse_button(event);
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
      input_child: &WidgetRefOwner,
      event: &MouseButtonsEventCtx,
   ) -> bool {
      //--------------------------------------------------
      let (flow, id, rect, _mouse_btn_num, _has_mouse_tracking) = input_child.data_for_dispatcher();
      //---------------------------------
      let is_enabled = flow.contains(StateFlags::IS_ENABLED);
      let is_inside = rect.is_inside(event.input.x, event.input.y);

      if !is_enabled || !is_inside {
         return false;
      }
      //--------------------------------------------------
      let children = match input_child.widget_mut() {
         Ok(mut child) => child.base_mut().children_mut().take_children(id),
         Err(e) => {
            log::error!("Can't borrow widget [{:?}] to process mouse button event!\n\t{:?}", id, e);
            return false;
         }
      };
      //--------------------------------------------------
      let mut accepted = false;
      for child in &children {
         if Self::emit_inner_mouse_button(dispatcher, child, event) {
            accepted = true;
            break;
         }
      }
      //--------------------------------------------------
      match input_child.widget_mut() {
         Ok(mut child) => {
            child.base_mut().children_mut().return_children(children, id);
            if !accepted {
               debug_assert!(
                  event.input.state != MouseState::Released,
                  "expected to be processed before"
               );
               child.base_mut().add_mouse_btn_num(1);
               accepted = child.on_mouse_button(event);
               dispatcher.widget_mouse_button = Some(input_child.as_ref());
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
      child: &WidgetRefOwner,
      event: &MouseWheelEventCtx,
   ) -> bool {
      //--------------------------------------------------
      let (flow, id, rect, _mouse_btn_num, _has_mouse_tracking) = child.data_for_dispatcher();
      //---------------------------------
      let is_enabled = flow.contains(StateFlags::IS_ENABLED);
      let is_inside = rect.is_inside(event.input.x, event.input.y);

      if !is_enabled || !is_inside {
         return false;
      }
      //--------------------------------------------------
      let children = match child.widget_mut() {
         Ok(mut child) => child.base_mut().children_mut().take_children(id),
         Err(e) => {
            log::error!("Can't borrow widget [{:?}] to process mouse wheel event!\n\t{:?}", id, e);
            return false;
         }
      };
      //--------------------------------------------------
      let mut accepted = false;
      for child in &children {
         if Self::emit_inner_mouse_wheel(dispatcher, child, event) {
            accepted = true;
            break;
         }
      }
      //--------------------------------------------------
      match child.widget_mut() {
         Ok(mut child) => {
            child.base_mut().children_mut().return_children(children, id);
            if !accepted {
               accepted = child.on_mouse_wheel(event);
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
      child: &WidgetRefOwner,
      event: &KeyboardEventCtx,
   ) -> bool {
      //--------------------------------------------------
      let (flow, id, _rect, _mouse_btn_num, _has_mouse_tracking) = child.data_for_dispatcher();
      //---------------------------------
      let is_enabled = flow.contains(StateFlags::IS_ENABLED);
      if !is_enabled {
         return false;
      }
      //--------------------------------------------------
      let children = match child.widget_mut() {
         Ok(mut child) => child.base_mut().children_mut().take_children(id),
         Err(e) => {
            log::error!("Can't borrow widget [{:?}] to process mouse wheel event!\n\t{:?}", id, e);
            return false;
         }
      };
      //--------------------------------------------------
      let mut accepted = false;
      for child in &children {
         if Self::emit_inner_keyboard(dispatcher, child, event) {
            accepted = true;
            break;
         }
      }
      //--------------------------------------------------
      match child.widget_mut() {
         Ok(mut child) => {
            child.base_mut().children_mut().return_children(children, id);
            if !accepted && child.on_keyboard(event) {
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
   fn inform_lost_children(dispatcher: &mut InnerDispatcher, children: &[WidgetRefOwner]) {
      for child in children {
         Self::emit_inner_lifecycle(
            dispatcher,
            child,
            &LifecycleEventCtx { state: LifecycleState::Destroy { unexpected: true } },
         );
      }
   }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
   use super::*;
   use crate::widgets::Widget;
   use sim_draw::m::Rect;
   use sim_input::mouse::MouseMoveInput;
   use sim_input::{DeviceId, Modifiers};

   //---------------------------------------------------

   pub struct TestWidget {
      pub mouse_enter: usize,
      pub mouse_leave: usize,
   }

   impl TestWidget {
      pub fn new(rect: Rect<f32>) -> WidgetRefOwner {
         Widget::inherit(
            |vt| {
               vt.on_mouse_cross = |w: &mut Widget<Self>, enter| match enter {
                  true => w.inherited_obj_mut().mouse_enter += 1,
                  false => w.inherited_obj_mut().mouse_leave += 1,
               };
               Self { mouse_enter: 0, mouse_leave: 0 }
            },
            |w| {
               w.set_rect(rect);
            },
         )
         .to_owner()
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

      let mut dispatcher = Dispatcher::new(Some(root.clone()), Runtime::default());
      dispatcher.widget().add_child(child.clone());
      //----------------------
      dispatcher.emit_mouse_move(&mouse_move_ctx(-10.0, 100.0));
      assert_eq!(
         root.widget().unwrap().inherited().downcast_ref::<TestWidget>().unwrap().mouse_enter,
         0
      );
      assert_eq!(
         root.widget().unwrap().inherited().downcast_ref::<TestWidget>().unwrap().mouse_leave,
         0
      );
      assert_eq!(
         child.widget().unwrap().inherited().downcast_ref::<TestWidget>().unwrap().mouse_enter,
         0
      );
      assert_eq!(
         child.widget().unwrap().inherited().downcast_ref::<TestWidget>().unwrap().mouse_leave,
         0
      );
      //----------------------
      dispatcher.emit_mouse_move(&mouse_move_ctx(25.0, 100.0));
      assert_eq!(
         root.widget().unwrap().inherited().downcast_ref::<TestWidget>().unwrap().mouse_enter,
         1
      );
      assert_eq!(
         root.widget().unwrap().inherited().downcast_ref::<TestWidget>().unwrap().mouse_leave,
         0
      );
      assert_eq!(
         child.widget().unwrap().inherited().downcast_ref::<TestWidget>().unwrap().mouse_enter,
         0
      );
      assert_eq!(
         child.widget().unwrap().inherited().downcast_ref::<TestWidget>().unwrap().mouse_leave,
         0
      );
      //----------------------
      dispatcher.emit_mouse_move(&mouse_move_ctx(75.0, 100.0));
      assert_eq!(
         root.widget().unwrap().inherited().downcast_ref::<TestWidget>().unwrap().mouse_enter,
         1
      );
      assert_eq!(
         root.widget().unwrap().inherited().downcast_ref::<TestWidget>().unwrap().mouse_leave,
         1
      );
      assert_eq!(
         child.widget().unwrap().inherited().downcast_ref::<TestWidget>().unwrap().mouse_enter,
         1
      );
      assert_eq!(
         child.widget().unwrap().inherited().downcast_ref::<TestWidget>().unwrap().mouse_leave,
         0
      );
      //----------------------
      dispatcher.emit_mouse_move(&mouse_move_ctx(25.0, 100.0));
      assert_eq!(
         root.widget().unwrap().inherited().downcast_ref::<TestWidget>().unwrap().mouse_enter,
         2
      );
      assert_eq!(
         root.widget().unwrap().inherited().downcast_ref::<TestWidget>().unwrap().mouse_leave,
         1
      );
      assert_eq!(
         child.widget().unwrap().inherited().downcast_ref::<TestWidget>().unwrap().mouse_enter,
         1
      );
      assert_eq!(
         child.widget().unwrap().inherited().downcast_ref::<TestWidget>().unwrap().mouse_leave,
         1
      );
      //----------------------
      dispatcher.emit_mouse_move(&mouse_move_ctx(-10.0, 100.0));
      assert_eq!(
         root.widget().unwrap().inherited().downcast_ref::<TestWidget>().unwrap().mouse_enter,
         2
      );
      assert_eq!(
         root.widget().unwrap().inherited().downcast_ref::<TestWidget>().unwrap().mouse_leave,
         2
      );
      assert_eq!(
         child.widget().unwrap().inherited().downcast_ref::<TestWidget>().unwrap().mouse_enter,
         1
      );
      assert_eq!(
         child.widget().unwrap().inherited().downcast_ref::<TestWidget>().unwrap().mouse_leave,
         1
      );
      //----------------------
   }

   //---------------------------------------------------

   #[test]
   fn sizes() {
      dbg!(std::mem::size_of::<Dispatcher>());
   }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
