use super::events::{
   KeyboardEventCtx, MouseButtonsEventCtx, MouseMoveEventCtx, MouseWheelEventCtx, UpdateEventCtx,
};
use super::{AppEnv, Dispatcher, SharedData, WidgetStrongRef};
use crate::backend::Resources;
use crate::{render::Canvas, theme::Theme};

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct Runtime {
   dispatcher: Dispatcher,
   app_env: AppEnv,
}

impl Runtime {
   pub fn new<RES>(
      root: Option<WidgetStrongRef>,
      app_env: AppEnv,
      theme: Theme,
      resources: RES,
   ) -> Self
   where
      RES: Resources + 'static,
   {
      Self { app_env, dispatcher: Dispatcher::new(root, SharedData::new(theme, resources)) }
   }
}

impl Runtime {
   pub fn emit_update(&mut self, event: &UpdateEventCtx) -> bool {
      self.dispatcher.emit_tick(&mut self.app_env, event)
   }

   pub fn emit_draw(&mut self, canvas: &mut Canvas, force: bool) {
      self.dispatcher.emit_draw(&mut self.app_env, canvas, force);
   }

   pub fn emit_mouse_move(&mut self, event: &MouseMoveEventCtx) -> bool {
      self.dispatcher.emit_mouse_move(event)
   }

   pub fn emit_mouse_button(&mut self, event: &MouseButtonsEventCtx) -> bool {
      self.dispatcher.emit_mouse_button(event)
   }

   pub fn emit_mouse_wheel(&mut self, event: &MouseWheelEventCtx) -> bool {
      self.dispatcher.emit_mouse_wheel(event)
   }

   pub fn emit_keyboard(&mut self, event: &KeyboardEventCtx) -> bool {
      self.dispatcher.emit_keyboard(event)
   }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
