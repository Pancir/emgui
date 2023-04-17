use crate::{core::events::DrawEventCtx, elements::Icon};
use bitflags::bitflags;
use sim_draw::{m::Rect, Canvas};
use std::borrow::Cow;

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

pub struct ButtonStyleState<'internal> {
   pub text: &'internal Option<Cow<'static, str>>,
   pub icon: &'internal Option<Icon>,
   pub toggle_num: u8,
   pub toggle: u8,
   pub options: ButtonStyleOption,
}

pub trait ButtonStyleSheet {
   fn rect(&self, state: &ButtonStyleState) -> Rect<f32>;
   fn draw(&self, state: &ButtonStyleState, canvas: &mut Canvas, _event: &DrawEventCtx);
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct ButtonStyle {}

impl ButtonStyleSheet for ButtonStyle {
   fn rect(&self, state: &ButtonStyleState) -> Rect<f32> {
      todo!()
   }

   fn draw(&self, state: &ButtonStyleState, canvas: &mut Canvas, _event: &DrawEventCtx) {
      todo!()
   }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
