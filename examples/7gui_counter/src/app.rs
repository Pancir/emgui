use sim_input::keyboard::KeyboardInput;
use sim_input::mouse::{MouseButtonsInput, MouseMoveInput, MouseWheelInput};
use sim_run::app::events::{DrawEvent, InitEvent, UpdateEvent};
use sim_run::app::IApp;
use sim_ui::core::control::Dispatcher;
use sim_ui::core::{AppEnv, Theme};
use std::cell::RefCell;
use std::rc::Rc;

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct App {
   app_env: AppEnv,
   dipatcher: Dispatcher,
}

impl App {
   pub fn new() -> Self {
      Self { app_env: AppEnv::new((), Theme::default()), dipatcher: Dispatcher::new(None) }
   }
}

impl IApp for App {
   fn on_init(&mut self, _event: InitEvent) {}

   fn on_update(&mut self, event: &UpdateEvent) {
      self.dipatcher.emit_tick(&mut self.app_env, event);
   }

   fn needs_draw(&self) -> bool {
      true
   }
   fn on_draw(&mut self, _ev: &DrawEvent) {}

   fn on_mouse_move(&mut self, _ev: &MouseMoveInput) {}
   fn on_mouse_button(&mut self, _ev: &MouseButtonsInput) {}
   fn on_mouse_wheel(&mut self, _ev: &MouseWheelInput) {}
   fn on_keyboard(&mut self, _ev: &KeyboardInput) {}

   fn on_final(&mut self) {}
}

////////////////////////////////////////////////////////////////////////////////////////////////////
