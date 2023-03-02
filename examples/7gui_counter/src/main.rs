//! https://eugenkiss.github.io/7guis/

mod app;

use crate::app::App;

use sim_draw::m::Rect;
use sim_run::app::Window;
use std::time::Duration;
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::prelude::*;

////////////////////////////////////////////////////////////////////////////////////////////////////

const TITLE: &str = "7GUI Counter example";

fn main() {
   //----------------------------------
   tracing_subscriber::fmt()
      // .with_span_events(FmtSpan::EXIT)
      .with_span_events(FmtSpan::NONE)
      .with_ansi(true)
      .with_target(true)
      .with_max_level(LevelFilter::TRACE)
      .init();

   tracing::info!(TITLE);
   //----------------------------------
   let app = App::new();
   //----------------------------------
   let rect = Rect::<i32>::new(300, 200, 512, 512);
   Window::new(false, 1.0, Duration::from_millis(100), true).run(TITLE.into(), rect, app).unwrap();
   //----------------------------------
}

////////////////////////////////////////////////////////////////////////////////////////////////////
