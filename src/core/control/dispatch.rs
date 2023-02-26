use super::*;

use crate::core::control::ChildrenVec;
use crate::core::events::{
   DrawEventCtx, KeyboardEventCtx, LayoutEventCtx, LifecycleEventCtx, MouseButtonsEventCtx,
   MouseMoveEventCtx, MouseWheelEventCtx, UpdateEventCtx,
};
use crate::core::IWidget;
use sim_draw::Canvas;
use std::cell::RefCell;
use std::rc::Rc;

////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn lifecycle(child: &Rc<RefCell<dyn IWidget>>, event: &LifecycleEventCtx) {
   let mut children = match child.try_borrow_mut() {
      Ok(mut child) => {
         let id = child.id();
         child.internal_mut().take_children(id)
      }
      Err(e) => {
         panic!("{}", e)
      }
   };

   if !children.is_empty() {
      lifecycle_children(&mut children, event);
   }

   match child.try_borrow_mut() {
      Ok(mut child) => {
         let id = child.id();
         child.internal_mut().set_children(children, id);
         child.emit_lifecycle(event)
      }
      Err(e) => {
         panic!("{}", e)
      }
   };
}

fn lifecycle_children(children: &mut ChildrenVec, event: &LifecycleEventCtx) {
   for child in children {
      lifecycle(&child, event);
   }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn layout(child: &Rc<RefCell<dyn IWidget>>, event: &LayoutEventCtx) {
   let mut children = match child.try_borrow_mut() {
      Ok(mut child) => {
         child.emit_layout(event);
         let id = child.id();
         child.internal_mut().take_children(id)
      }
      Err(e) => {
         panic!("{}", e)
      }
   };

   if !children.is_empty() {
      layout_children(&mut children, event);
   }

   let mut bor = child.try_borrow_mut().unwrap_or_else(|e| panic!("{}", e));
   let id = bor.id();
   bor.internal_mut().set_children(children, id);
}

fn layout_children(children: &mut ChildrenVec, event: &LayoutEventCtx) {
   for child in children {
      layout(&child, event);
   }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn update(child: &Rc<RefCell<dyn IWidget>>, event: &UpdateEventCtx) {
   //--------------------------------------------------
   // # Safety
   // It seems it is quite safe, we just read simple copiable variables.
   // Just in case in debug mode we check availability.
   debug_assert!(child.try_borrow_mut().is_ok());
   let (flow, id) =
      unsafe { ((*child.as_ptr()).internal().control_flow.get(), (*child.as_ptr()).id()) };
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
   let is_continue = is_self_delete || is_self_update || is_children_delete || is_children_update;

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
         lifecycle(&child, &LifecycleEventCtx::Delete);
         //---------------------------------
         false
      });
   }
   //--------------------------------------------------
   // UPDATE CHILDREN

   if is_children_update {
      update_children(&mut children, event);
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
         for child in children {
            lifecycle(&child, &LifecycleEventCtx::Delete);
         }
         //-----------------------
         log::error!(
            "Can't borrow widget [{:?}] to process update finalization! Children ARE LOST!\n\t{}",
            id,
            e
         );
         return;
      }
   };
}

fn update_children(children: &mut ChildrenVec, event: &UpdateEventCtx) {
   for child in children {
      update(&child, event);
   }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn draw(child: &Rc<RefCell<dyn IWidget>>, canvas: &mut Canvas, force: bool) {
   draw_child(child, canvas, &DrawEventCtx {}, force)
}

pub fn draw_child(
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
   let (flow, id) =
      unsafe { ((*child.as_ptr()).internal().control_flow.get(), (*child.as_ptr()).id()) };
   //---------------------------------
   let is_self_draw = flow.contains(ControlFlow::SELF_DRAW);
   let is_visible = flow.contains(ControlFlow::IS_VISIBLE);
   let id_draw = is_visible && (is_self_draw || flow.contains(ControlFlow::CHILDREN_DRAW));

   if !id_draw {
      return;
   }
   //--------------------------------------------------
   let (mut children, regions) = match child.try_borrow_mut() {
      Ok(mut child) => {
         let internal = child.internal_mut();

         if is_visible && (is_self_draw || force) {
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

   draw_children(&mut children, canvas, event, force);

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
         log::error!(
            "Can't borrow widget [{:?}] to process draw finalization! Children ARE LOST!\n\t{}",
            id,
            e
         );
         return;
      }
   };
}

fn draw_children(
   children: &mut ChildrenVec,
   canvas: &mut Canvas,
   event: &DrawEventCtx,
   force: bool,
) {
   for child in children {
      draw_child(&child, canvas, event, force);
   }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn mouse_move(child: &Rc<RefCell<dyn IWidget>>, event: &MouseMoveEventCtx) -> bool {
   let mut children = match child.try_borrow_mut() {
      Ok(mut child) => {
         let id = child.id();
         child.internal_mut().take_children(id)
      }
      Err(e) => {
         panic!("{}", e)
      }
   };

   if !children.is_empty() {
      mouse_move_children(&mut children, event);
   }

   match child.try_borrow_mut() {
      Ok(mut child) => {
         let id = child.id();
         child.internal_mut().set_children(children, id);
         if child.emit_mouse_move(event) {
            return true;
         }
      }
      Err(e) => {
         panic!("{}", e)
      }
   };

   false
}

fn mouse_move_children(children: &mut ChildrenVec, event: &MouseMoveEventCtx) -> bool {
   for child in children {
      if mouse_move(&child, event) {
         return true;
      }
   }
   false
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn mouse_button(child: &Rc<RefCell<dyn IWidget>>, event: &MouseButtonsEventCtx) -> bool {
   let mut children = match child.try_borrow_mut() {
      Ok(mut child) => {
         let id = child.id();
         child.internal_mut().take_children(id)
      }
      Err(e) => {
         panic!("{}", e)
      }
   };

   if !children.is_empty() {
      mouse_button_children(&mut children, event);
   }

   match child.try_borrow_mut() {
      Ok(mut child) => {
         let id = child.id();
         child.internal_mut().set_children(children, id);
         if child.emit_mouse_button(event) {
            return true;
         }
      }
      Err(e) => {
         panic!("{}", e)
      }
   };

   false
}

fn mouse_button_children(children: &mut ChildrenVec, event: &MouseButtonsEventCtx) -> bool {
   for child in children {
      if mouse_button(&child, event) {
         return true;
      }
   }
   false
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn mouse_wheel(child: &Rc<RefCell<dyn IWidget>>, event: &MouseWheelEventCtx) -> bool {
   let mut children = match child.try_borrow_mut() {
      Ok(mut child) => {
         let id = child.id();
         child.internal_mut().take_children(id)
      }
      Err(e) => {
         panic!("{}", e)
      }
   };

   if !children.is_empty() {
      mouse_wheel_children(&mut children, event);
   }

   match child.try_borrow_mut() {
      Ok(mut child) => {
         let id = child.id();
         child.internal_mut().set_children(children, id);
         if child.emit_mouse_wheel(event) {
            return true;
         }
      }
      Err(e) => {
         panic!("{}", e)
      }
   };

   false
}

fn mouse_wheel_children(children: &mut ChildrenVec, event: &MouseWheelEventCtx) -> bool {
   for child in children {
      if mouse_wheel(&child, event) {
         return true;
      }
   }
   false
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn keyboard(child: &Rc<RefCell<dyn IWidget>>, event: &KeyboardEventCtx) -> bool {
   let mut children = match child.try_borrow_mut() {
      Ok(mut child) => {
         let id = child.id();
         child.internal_mut().take_children(id)
      }
      Err(e) => {
         panic!("{}", e)
      }
   };

   if !children.is_empty() {
      mouse_keyboard_children(&mut children, event);
   }

   match child.try_borrow_mut() {
      Ok(mut child) => {
         let id = child.id();
         child.internal_mut().set_children(children, id);
         if child.emit_keyboard(event) {
            return true;
         }
      }
      Err(e) => {
         panic!("{}", e)
      }
   };

   false
}

fn mouse_keyboard_children(children: &mut ChildrenVec, event: &KeyboardEventCtx) -> bool {
   for child in children {
      if keyboard(&child, event) {
         return true;
      }
   }
   false
}

////////////////////////////////////////////////////////////////////////////////////////////////////
