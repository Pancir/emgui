use super::ButtonState;
use crate::core::events::DrawEventCtx;
use bitflags::bitflags;
use sim_draw::{m::Rect, Canvas};

////////////////////////////////////////////////////////////////////////////////////////////////////

bitflags! {
   #[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
   pub struct ButtonStyleOption: u8 {
      const HAS_MENU = 1<<0;
      const DEFAULT = 1<<1;
      const AUTO_DEFAULT = 1<<2;
      const FOCUSED = 1<<3;
      const MOUSE_HOVER = 1<<4;
      const IS_DOWN = 1<<5;
   }
}

pub trait ButtonStyleSheet {
   fn rect(&self, state: &ButtonState) -> Rect<f32>;
   fn draw(&self, state: &ButtonState, canvas: &mut Canvas, _event: &DrawEventCtx);
}

////////////////////////////////////////////////////////////////////////////////////////////////////
