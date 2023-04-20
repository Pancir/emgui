use bitflags::bitflags;

////////////////////////////////////////////////////////////////////////////////////////////////////

bitflags! {
   /// Control flow for events.
   #[derive(Debug, Clone, Copy, PartialEq, Eq)]
   pub(crate) struct StateFlags: u16 {
      //-----------------------------

      /// Call update for current widget.
      const SELF_UPDATE     = 1<<0;

      /// Some children need update.
      const CHILDREN_UPDATE = 1<<1;

      //-----------------------------

      /// Call update for current widget.
      const SELF_DRAW     = 1<<2;

      /// Some children need update.
      const CHILDREN_DRAW = 1<<3;

      //-----------------------------

      /// Request to delete this widget.
      const SELF_DELETE     = 1<<4;

      /// Widget has one or more children to delete.
      const CHILDREN_DELETE = 1<<5;

      //-----------------------------

      /// Visible state.
      const IS_VISIBLE = 1<<6;

      /// Set if interaction events are desired like mouse and keyboard ones..
      const IS_ENABLED = 1<<7;

      /// Set if background has transparent pixels.
      const IS_TRANSPARENT = 1<<8;

      //-----------------------------

      /// It is set when mouse is over the widgets rectangle.
      const IS_OVER = 1<<9;

      /// Mouse tracking state.
      const HAS_MOUSE_TRACKING = 1<<10;

      //-----------------------------

      /// This flag is set when this widget has received focus.
      const HAS_FOCUS = 1<<11;

      /// This flag is set when a child in the children hierarchy has received focus.
      const HAS_CHILD_FOCUS = 1<<12;

      //-----------------------------

      const INIT = Self::SELF_DRAW.bits()|Self::CHILDREN_DRAW.bits()|Self::SELF_UPDATE.bits()|Self::CHILDREN_UPDATE.bits()|Self::IS_VISIBLE.bits()|Self::IS_ENABLED.bits();
   }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
