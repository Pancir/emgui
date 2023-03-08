use crate::core::widget_base::Dispatcher;
use crate::core::{AppEnv, Theme};
use sim_input::keyboard::KeyboardInput;
use sim_input::mouse::{MouseButtonsInput, MouseMoveInput, MouseWheelInput};
use sim_run::app::events::{DrawEvent, InitEvent, UpdateEvent};
use sim_run::app::IApp;

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct App {
   app_env: AppEnv,
   dispatcher: Dispatcher,
}

impl App {
   pub fn new(app_env: AppEnv, dispatcher: Dispatcher) -> Self {
      Self { app_env, dispatcher }
   }
}

impl IApp for App {
   fn on_init(&mut self, _event: InitEvent) {}

   fn on_update(&mut self, event: &UpdateEvent) {
      self.dispatcher.emit_tick(
         &mut self.app_env,
         &sim_run::UpdateEvent {
            call_num: event.call_num,
            delta: Second(event.delta.as_secs_f32()),
            abs_time: event.abs_time,
         },
      );
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
