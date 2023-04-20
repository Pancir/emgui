use std::any::{Any, TypeId};

use crate::{
   elements::Icon,
   render::{Painter, RenderObject, RenderObjectBase},
   theme::Theme,
};
use anyhow::bail;
use m::Box2;

////////////////////////////////////////////////////////////////////////////////////////////////////

/// Button render object data.
pub struct ButtonRenderObjectData<'refs> {
   pub text: Option<&'refs str>,
   pub icon: Option<&'refs Icon>,
   pub bounds: Box2<f32>,
   pub is_enabled: bool,
   pub is_hover: bool,
   pub is_active: bool,
   pub has_focus: bool,
   pub has_menu: bool,
   pub toggle_num: u8,
   pub toggle_curr: u8,
}

/// Base type for all button render objects.
pub trait ButtonRenderObject: for<'refs> RenderObject<ButtonRenderObjectData<'refs>> {}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Default)]
pub struct ButtonRender {}

impl RenderObjectBase for ButtonRender {
   fn can_render(&self, type_id: TypeId) -> bool {
      type_id == TypeId::of::<ButtonRenderObjectData>()
   }

   fn render_any_bounds(&self, theme: &Theme, data: &dyn Any) -> Option<Box2<f32>> {
      data.downcast_ref::<ButtonRenderObjectData>().map(|data| self.render_bounds(theme, data))
   }

   fn render_any(
      &self,
      theme: &Theme,
      data: &dyn Any,
      painter: &mut Painter,
   ) -> anyhow::Result<()> {
      if let Some(data) = data.downcast_ref::<ButtonRenderObjectData>() {
         self.render(theme, data, painter);
         Ok(())
      } else {
         bail!("Render data is not supported by the renderer")
      }
   }
}

impl ButtonRenderObject for ButtonRender {}

impl ButtonRender {
   pub fn new_normal() -> Self {
      ButtonRender::default()
   }

   pub fn new_accent() -> Self {
      ButtonRender::default()
   }
}

impl RenderObject<ButtonRenderObjectData<'_>> for ButtonRender {
   fn render_bounds(&self, _theme: &Theme, data: &ButtonRenderObjectData) -> Box2<f32> {
      data.bounds
   }

   fn render(&self, _theme: &Theme, _data: &ButtonRenderObjectData, _painter: &mut Painter) {
      // FIXME draw

      // painter.set_brush(Brush::new_color(Rgba::GREEN.with_alpha_mul(0.5)));

      // if data.is_hover {
      //    painter.set_brush(Brush::new_color(Rgba::AMBER));
      // }

      // if data.is_active {
      //    painter.set_brush(Brush::new_color(Rgba::RED));
      // }

      // painter.fill(&data.bounds);

      // painter.set_pen(Pen::new().with_width(2.0).with_color(Rgba::BLACK));
      // painter.stroke(&data.bounds);

      // if let Some(txt) = data.text {
      //    // TODO continue
      //    painter.set_brush(Brush::new_color(Rgba::BLACK));
      //    // painter.set_text_paint(self.paint.clone());
      //    painter.fill_text_line(
      //       txt,
      //       data.bounds.center(),
      //       TextAlign::new().center().middle().tight(),
      //    );
      // }
   }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
