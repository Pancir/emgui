use crate::core::ctx::{DrawCtx, MouseButtonsCtx, MouseMoveCtx};
use crate::core::{Geometry, IWidget, Point, Rect, Size, WidgetPodHandle};
use crate::widgets::state::IState;
use input::mouse::{MouseButton, MouseState};
use layout::{AlignH, AlignV};
use std::borrow::Cow;

////////////////////////////////////////////////////////////////////////////////////////////////////

/// Base trait for all states of text buttons.
pub trait ITextButtonState: IState {
   fn on_click(&mut self);
}

////////////////////////////////////////////////////////////////////////////////////////////////////

/// Default state for text buttons
#[derive(Default)]
pub struct TextButtonState {
   on_click: Option<Box<dyn FnMut()>>,
}

impl TextButtonState {
   pub fn do_on_click(mut self, cb: impl FnMut() + 'static) -> Self {
      self.on_click = Some(Box::new(cb));
      self
   }
}

impl ITextButtonState for TextButtonState {
   fn on_click(&mut self) {
      if let Some(f) = self.on_click.as_mut() {
         (*f)()
      }
   }
}

impl IState for TextButtonState {}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct TextButton<State = TextButtonState>
where
   State: 'static + ITextButtonState,
{
   handle: WidgetPodHandle,
   state: State,
   text: Cow<'static, str>,
   geom: Geometry,
   is_toggle: bool,
   is_hover: bool,
   is_down: bool,
   v_on_draw: fn(&mut Self, &mut DrawCtx),
   v_on_mouse_move: fn(&mut Self, &mut MouseMoveCtx),
   v_on_mouse_button: fn(&mut Self, &mut MouseButtonsCtx),
}

impl Default for TextButton {
   fn default() -> Self {
      Self {
         handle: WidgetPodHandle::empty(),
         state: TextButtonState::default(),
         text: Cow::Owned(String::default()),
         geom: Geometry::default(),
         is_toggle: false,
         is_hover: false,
         is_down: false,
         v_on_draw: Self::on_draw,
         v_on_mouse_move: Self::on_mouse_move,
         v_on_mouse_button: Self::on_mouse_button,
      }
   }
}

impl<State> TextButton<State>
where
   State: 'static + ITextButtonState,
{
   pub fn state(&self) -> &State {
      &self.state
   }

   pub fn state_mut(&mut self) -> &mut State {
      &mut self.state
   }

   pub fn set_text(&mut self, text: impl Into<Cow<'static, str>>) {
      self.text = text.into()
   }

   pub fn text(&self) -> &str {
      self.text.as_ref()
   }

   pub fn set_max_size(&mut self, size: Size) {
      if self.geom.set_max_size(size) {
         self.handle.request_draw();
      }
   }

   pub fn set_min_size(&mut self, size: Size) {
      if self.geom.set_min_size(size) {
         self.handle.request_draw();
      }
   }

   pub fn set_toggle(&mut self, state: bool) {
      self.is_toggle = state;
   }

   pub fn is_down(&self) -> bool {
      self.is_down
   }

   pub fn is_hover(&self) -> bool {
      self.is_hover
   }
}

impl<State> TextButton<State>
where
   State: 'static + ITextButtonState,
{
   fn on_draw(w: &mut Self, ctx: &mut DrawCtx) {
      let theme = &ctx.env.theme();
      let canvas = &mut ctx.canvas;
      let color = if w.is_down {
         theme.button.presses_color
      } else {
         if w.is_hover {
            theme.button.hover_color
         } else {
            theme.button.color
         }
      };
      canvas.set_color(color);
      canvas.set_white_texture();
      canvas.tris(&w.geom.rect);

      canvas.set_font(None);
      canvas.set_color(theme.button.text_color);
      canvas.text_aligned(
         &w.text,
         Point::from(w.geom.rect.center()),
         AlignH::Center,
         AlignV::Center,
      )
   }

   fn on_mouse_move(w: &mut Self, ctx: &mut MouseMoveCtx) {
      let is_inside = w.geom.rect.is_inside(ctx.input.x, ctx.input.y);
      if w.is_hover != is_inside {
         w.handle.request_draw();
         w.is_hover = is_inside;
      }
   }

   fn on_mouse_button(w: &mut Self, ctx: &mut MouseButtonsCtx) {
      let is_down = w.is_hover
         && ctx.input.button == MouseButton::Left
         && ctx.input.state == MouseState::Pressed;

      if w.is_toggle {
         unimplemented!()
      }

      if w.is_down != is_down {
         w.handle.request_draw();
         w.is_down = is_down;
      }
   }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

impl IWidget for TextButton {
   fn set_pod_handle(&mut self, handle: WidgetPodHandle) {
      debug_assert!(self.handle.is_empty(), "Pod handle has already been set");
      self.handle = handle;
   }

   fn geometry(&self) -> &Geometry {
      &self.geom
   }

   fn set_rect(&mut self, rect: Rect) {
      if self.geom.set_rect(rect) {
         self.handle.request_draw();
      }
   }

   fn on_draw(&mut self, ctx: &mut DrawCtx) {
      (self.v_on_draw)(self, ctx)
   }

   fn on_mouse_move(&mut self, ctx: &mut MouseMoveCtx) {
      (self.v_on_mouse_move)(self, ctx)
   }

   fn on_mouse_button(&mut self, ctx: &mut MouseButtonsCtx) {
      (self.v_on_mouse_button)(self, ctx)
   }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
