#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(clippy::single_match)]

const WINDOW_TITLE: &str = "Hello Window";

use beryllium::{
  events::Event,
  init::InitFlags,
  video::{CreateWinArgs, GlContextFlags, GlProfile},
  *,
};

fn main() {
  let sdl = Sdl::init(InitFlags::EVERYTHING);
  sdl.set_gl_context_major_version(3).unwrap();
  sdl.set_gl_context_minor_version(3).unwrap();
  sdl.set_gl_profile(GlProfile::Core).unwrap();
  let mut flags = GlContextFlags::default();
  if cfg!(target_os = "macos") {
    flags |= GlContextFlags::FORWARD_COMPATIBLE;
  }
  if cfg!(debug_asserts) {
    flags |= GlContextFlags::DEBUG;
  }
  sdl.set_gl_context_flags(flags).unwrap();

  let _win = sdl
    .create_gl_window(CreateWinArgs {
      title: WINDOW_TITLE,
      width: 800,
      height: 600,
      ..Default::default()
    })
    .expect("couldn't make a window and context");

  'main_loop: loop {
    // handle events this frame
    while let Some((event, _timestamp)) = sdl.poll_events() {
      match event {
        Event::Quit => break 'main_loop,
        _ => (),
      }
    }
    // now the events are clear.

    // here's where we could change the world state and draw.
  }
}
