/*
**  Copyright (C) 2022, StepToSky
**  All rights reserved
**
**  Contacts: www.steptosky.com
*/

use super::{DeviceId, Modifiers};

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum MouseState {
   Pressed,
   Released,
}

#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum MouseButton {
   Left,
   Right,
   Middle,
   Other(u16),
}

#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MouseButtonsInput {
   pub device_id: DeviceId,
   pub state: MouseState,
   pub button: MouseButton,
   pub modifiers: Modifiers,
   pub x: f32,
   pub y: f32,
}

#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MouseWheelInput {
   pub device_id: DeviceId,
   pub delta_x: f32,
   pub delta_y: f32,
   pub modifiers: Modifiers,
   pub x: f32,
   pub y: f32,
}

#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MouseMoveInput {
   pub device_id: DeviceId,
   pub modifiers: Modifiers,
   pub x: f32,
   pub y: f32,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
