use crate::widgets::children::ChildrenVec;
use crate::widgets::events::{
   KeyboardEvent, LayoutEvent, LifecycleEvent, MouseButtonsEvent, MouseMoveEvent, MouseWheelEvent,
   UpdateEvent,
};
use crate::widgets::IWidget;
use sim_draw::Canvas;
use std::cell::RefCell;
use std::rc::Rc;

////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn lifecycle(child: &Rc<RefCell<dyn IWidget>>, event: &LifecycleEvent) {
   let mut children = match child.try_borrow_mut() {
      Ok(mut child) => {
         child.emit_lifecycle(event);
         let id = child.id();
         child.children_mut().take(id)
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
   bor.children_mut().set(children, id);
}

fn lifecycle_children(children: &mut ChildrenVec, event: &LifecycleEvent) {
   for child in children {
      lifecycle(&child, event);
   }
}

//------------------------------------------------------------------------------------------------//

pub fn layout(child: &Rc<RefCell<dyn IWidget>>, event: &LayoutEvent) {
   let mut children = match child.try_borrow_mut() {
      Ok(mut child) => {
         child.emit_layout(event);
         let id = child.id();
         child.children_mut().take(id)
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
   bor.children_mut().set(children, id);
}

fn layout_children(children: &mut ChildrenVec, event: &LayoutEvent) {
   for child in children {
      layout(&child, event);
   }
}

//------------------------------------------------------------------------------------------------//

pub fn update(child: &Rc<RefCell<dyn IWidget>>, event: &UpdateEvent) {
   let mut children = match child.try_borrow_mut() {
      Ok(mut child) => {
         child.emit_update(event);
         let id = child.id();
         child.children_mut().take(id)
      }
      Err(e) => {
         panic!("{}", e)
      }
   };

   if !children.is_empty() {
      update_children(&mut children, event);
   }

   let mut bor = child.try_borrow_mut().unwrap_or_else(|e| panic!("{}", e));
   let id = bor.id();
   bor.children_mut().set(children, id);
}

fn update_children(children: &mut ChildrenVec, event: &UpdateEvent) {
   for child in children {
      update(&child, event);
   }
}

//------------------------------------------------------------------------------------------------//

pub fn draw(child: &Rc<RefCell<dyn IWidget>>, canvas: &mut Canvas, force: bool) {
   let (mut children, is_draw) = match child.try_borrow_mut() {
      Ok(mut child) => {
         let is_draw = child.is_visible() || force;
         if is_draw {
            child.emit_draw(canvas);
         }
         let id = child.id();
         (child.children_mut().take(id), is_draw)
      }
      Err(e) => {
         panic!("{}", e)
      }
   };

   if is_draw {
      draw_children(&mut children, canvas, force);
   }

   let mut bor = child.try_borrow_mut().unwrap_or_else(|e| panic!("{}", e));
   let id = bor.id();
   bor.children_mut().set(children, id);
}

fn draw_children(children: &mut ChildrenVec, canvas: &mut Canvas, force: bool) {
   for child in children {
      draw(&child, canvas, force);
   }
}

//------------------------------------------------------------------------------------------------//

pub fn mouse_move(child: &Rc<RefCell<dyn IWidget>>, event: &MouseMoveEvent) -> bool {
   let mut children = match child.try_borrow_mut() {
      Ok(mut child) => {
         let id = child.id();
         child.children_mut().take(id)
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
         child.children_mut().set(children, id);
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

fn mouse_move_children(children: &mut ChildrenVec, event: &MouseMoveEvent) -> bool {
   for child in children {
      if mouse_move(&child, event) {
         return true;
      }
   }
   false
}

//------------------------------------------------------------------------------------------------//

pub fn mouse_button(child: &Rc<RefCell<dyn IWidget>>, event: &MouseButtonsEvent) -> bool {
   let mut children = match child.try_borrow_mut() {
      Ok(mut child) => {
         let id = child.id();
         child.children_mut().take(id)
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
         child.children_mut().set(children, id);
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

fn mouse_button_children(children: &mut ChildrenVec, event: &MouseButtonsEvent) -> bool {
   for child in children {
      if mouse_button(&child, event) {
         return true;
      }
   }
   false
}

//------------------------------------------------------------------------------------------------//

pub fn mouse_wheel(child: &Rc<RefCell<dyn IWidget>>, event: &MouseWheelEvent) -> bool {
   let mut children = match child.try_borrow_mut() {
      Ok(mut child) => {
         let id = child.id();
         child.children_mut().take(id)
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
         child.children_mut().set(children, id);
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

fn mouse_wheel_children(children: &mut ChildrenVec, event: &MouseWheelEvent) -> bool {
   for child in children {
      if mouse_wheel(&child, event) {
         return true;
      }
   }
   false
}

//------------------------------------------------------------------------------------------------//

pub fn keyboard(child: &Rc<RefCell<dyn IWidget>>, event: &KeyboardEvent) -> bool {
   let mut children = match child.try_borrow_mut() {
      Ok(mut child) => {
         let id = child.id();
         child.children_mut().take(id)
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
         child.children_mut().set(children, id);
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

fn mouse_keyboard_children(children: &mut ChildrenVec, event: &KeyboardEvent) -> bool {
   for child in children {
      if keyboard(&child, event) {
         return true;
      }
   }
   false
}

////////////////////////////////////////////////////////////////////////////////////////////////////
