#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(clippy::single_match)]

use beryllium::*;

fn main() {
  let sdl = SDL::init(InitFlags::Everything).expect("couldn't start SDL");
  sdl.gl_set_attribute(SdlGlAttr::MajorVersion, 3).unwrap();
  sdl.gl_set_attribute(SdlGlAttr::MinorVersion, 3).unwrap();
  sdl.gl_set_attribute(SdlGlAttr::Profile, GlProfile::Core).unwrap();
  sdl
    .gl_set_attribute(SdlGlAttr::Flags, ContextFlag::ForwardCompatible)
    .unwrap();

  let _win = sdl
    .create_gl_window(
      "Hello Window",
      WindowPosition::Centered,
      800,
      600,
      WindowFlags::Shown,
    )
    .expect("couldn't make a window and context");

  'main_loop: loop {
    while let Some(event) = sdl.poll_events().and_then(Result::ok) {
      // handle events this frame
      match event {
        Event::Quit(_) => break 'main_loop,
        _ => (),
      }

      // here's where we could change the world state and draw.
    }
  }
}
