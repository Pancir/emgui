/*
**  Copyright (C) 2022, StepToSky
**  All rights reserved
**
**  Contacts: www.steptosky.com
*/

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Default, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DeviceId(u32);

impl DeviceId {
   pub fn from_raw(v: u32) -> Self {
      Self(v)
   }

   pub fn raw(self) -> u32 {
      self.0
   }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
