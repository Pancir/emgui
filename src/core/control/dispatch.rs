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
         child.emit_lifecycle(event);
         let id = child.id();
         child.internal_mut().take(id)
      }
      Err(e) => {
         panic!("{}", e)
      }
   };

   if !children.is_empty() {
      lifecycle_children(&mut children, event);
   }

   let mut bor = child.try_borrow_mut().unwrap_or_else(|e| panic!("{}", e));
   let id = bor.id();
   bor.internal_mut().set(children, id);
}

fn lifecycle_children(children: &mut ChildrenVec, event: &LifecycleEventCtx) {
   for child in children {
      lifecycle(&child, event);
   }
}

//------------------------------------------------------------------------------------------------//

pub fn layout(child: &Rc<RefCell<dyn IWidget>>, event: &LayoutEventCtx) {
   let mut children = match child.try_borrow_mut() {
      Ok(mut child) => {
         child.emit_layout(event);
         let id = child.id();
         child.internal_mut().take(id)
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
   bor.internal_mut().set(children, id);
}

fn layout_children(children: &mut ChildrenVec, event: &LayoutEventCtx) {
   for child in children {
      layout(&child, event);
   }
}

//------------------------------------------------------------------------------------------------//

pub fn update(child: &Rc<RefCell<dyn IWidget>>, event: &UpdateEventCtx) {
   let (mut children, update) = match child.try_borrow_mut() {
      Ok(mut child) => {
         let update = child.needs_update(true);
         if update {
            child.emit_update(event);
         }
         let id = child.id();
         (child.internal_mut().take(id), update)
      }
      Err(e) => {
         panic!("{}", e)
      }
   };

   if update {
      update_children(&mut children, event);
   }

   let mut bor = child.try_borrow_mut().unwrap_or_else(|e| panic!("{}", e));
   let id = bor.id();
   bor.internal_mut().set(children, id);
}

fn update_children(children: &mut ChildrenVec, event: &UpdateEventCtx) {
   for child in children {
      update(&child, event);
   }
}

//------------------------------------------------------------------------------------------------//

pub fn draw(child: &Rc<RefCell<dyn IWidget>>, canvas: &mut Canvas, force: bool) {
   draw_child(child, canvas, &DrawEventCtx {}, force)
}

pub fn draw_child(
   child: &Rc<RefCell<dyn IWidget>>,
   canvas: &mut Canvas,
   event: &DrawEventCtx,
   force: bool,
) {
   let (mut children, is_draw) = match child.try_borrow_mut() {
      Ok(mut child) => {
         let is_draw = (child.needs_draw(true) && child.is_visible()) || force;
         if is_draw {
            child.emit_draw(canvas, event);
         }
         let id = child.id();
         (child.internal_mut().take(id), is_draw)
      }
      Err(e) => {
         panic!("{}", e)
      }
   };

   if is_draw {
      draw_children(&mut children, canvas, event, force);
   }

   let mut bor = child.try_borrow_mut().unwrap_or_else(|e| panic!("{}", e));
   let id = bor.id();
   bor.internal_mut().set(children, id);
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

//------------------------------------------------------------------------------------------------//

pub fn mouse_move(child: &Rc<RefCell<dyn IWidget>>, event: &MouseMoveEventCtx) -> bool {
   let mut children = match child.try_borrow_mut() {
      Ok(mut child) => {
         let id = child.id();
         child.internal_mut().take(id)
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
         child.internal_mut().set(children, id);
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

//------------------------------------------------------------------------------------------------//

pub fn mouse_button(child: &Rc<RefCell<dyn IWidget>>, event: &MouseButtonsEventCtx) -> bool {
   let mut children = match child.try_borrow_mut() {
      Ok(mut child) => {
         let id = child.id();
         child.internal_mut().take(id)
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
         child.internal_mut().set(children, id);
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

//------------------------------------------------------------------------------------------------//

pub fn mouse_wheel(child: &Rc<RefCell<dyn IWidget>>, event: &MouseWheelEventCtx) -> bool {
   let mut children = match child.try_borrow_mut() {
      Ok(mut child) => {
         let id = child.id();
         child.internal_mut().take(id)
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
         child.internal_mut().set(children, id);
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

//------------------------------------------------------------------------------------------------//

pub fn keyboard(child: &Rc<RefCell<dyn IWidget>>, event: &KeyboardEventCtx) -> bool {
   let mut children = match child.try_borrow_mut() {
      Ok(mut child) => {
         let id = child.id();
         child.internal_mut().take(id)
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
         child.internal_mut().set(children, id);
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
