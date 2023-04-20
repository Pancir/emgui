/*
**  Copyright (C) 2022, StepToSky
**  All rights reserved
**
**  Contacts: www.steptosky.com
*/

use bitflags::bitflags;

////////////////////////////////////////////////////////////////////////////////////////////////////

bitflags! {
    /// Represents the current state of the keyboard modifiers
    ///
    /// Each flag represents a modifier and is set if this modifier is active.
    #[derive(Default)]
    pub struct Modifiers: u32 {
        // left and right modifiers are currently commented out, but we should be able to support
        // them in a future release
        /// The "shift" key.
        const SHIFT = 0b100;
        // const LSHIFT = 0b010;
        // const RSHIFT = 0b001;
        /// The "control" key.
        const CTRL = 0b100 << 3;
        // const LCTRL = 0b010 << 3;
        // const RCTRL = 0b001 << 3;
        /// The "alt" key.
        const ALT = 0b100 << 6;
        // const LALT = 0b010 << 6;
        // const RALT = 0b001 << 6;
        /// This is the "windows" key on PC and "command" key on Mac.
        const VENDOR = 0b100 << 9;
        // const LLOGO = 0b010 << 9;
        // const RLOGO = 0b001 << 9;
    }
}

impl Modifiers {
   /// Returns `true` if the shift key is pressed.
   pub fn shift(&self) -> bool {
      self.intersects(Self::SHIFT)
   }
   /// Returns `true` if the control key is pressed.
   pub fn ctrl(&self) -> bool {
      self.intersects(Self::CTRL)
   }
   /// Returns `true` if the alt key is pressed.
   pub fn alt(&self) -> bool {
      self.intersects(Self::ALT)
   }
   /// Returns `true` if the logo key is pressed.
   pub fn vendor(&self) -> bool {
      self.intersects(Self::VENDOR)
   }
}

#[cfg(feature = "serde")]
mod modifiers_serde {
   use super::Modifiers;
   use serde::{Deserialize, Deserializer, Serialize, Serializer};

   #[derive(Default, Serialize, Deserialize)]
   #[serde(default)]
   #[serde(rename = "ModifiersState")]
   pub struct ModifiersStateSerialize {
      pub shift: bool,
      pub ctrl: bool,
      pub alt: bool,
      pub vendor: bool,
   }

   impl Serialize for Modifiers {
      fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
      where
         S: Serializer,
      {
         let s = ModifiersStateSerialize {
            shift: self.shift(),
            ctrl: self.ctrl(),
            alt: self.alt(),
            vendor: self.vendor(),
         };
         s.serialize(serializer)
      }
   }

   impl<'de> Deserialize<'de> for Modifiers {
      fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
      where
         D: Deserializer<'de>,
      {
         let ModifiersStateSerialize { shift, ctrl, alt, vendor } =
            ModifiersStateSerialize::deserialize(deserializer)?;
         let mut m = Modifiers::empty();
         m.set(Modifiers::SHIFT, shift);
         m.set(Modifiers::CTRL, ctrl);
         m.set(Modifiers::ALT, alt);
         m.set(Modifiers::VENDOR, vendor);
         Ok(m)
      }
   }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
