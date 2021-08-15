# Creating A Window

This part of the tutorial is very library specific, so I won't focus on it too
much. Basically, we have to open a window, and we also need a GL context to go
with that window. The details for this depend on what OS and windowing system
you're using. In my case, `beryllium` is based on SDL2, so we have a nice
cross-platform abstraction going for us.

## Pre-Window Setup

On most platforms, you have to specify that you'll be using GL _before_ you create the window, so that the window itself can be created with the correct settings to support GL once it's made.

First we turn on SDL itself:

```rust
use beryllium::*;

fn main() {
  let sdl = SDL::init(InitFlags::Everything).expect("couldn't start SDL");
```

Then we set some attributes for the [OpenGL
Context](https://www.khronos.org/opengl/wiki/OpenGL_Context) that we want to
use:

```rust
  sdl.gl_set_attribute(SdlGlAttr::MajorVersion, 3).unwrap();
  sdl.gl_set_attribute(SdlGlAttr::MinorVersion, 3).unwrap();
  sdl.gl_set_attribute(SdlGlAttr::Profile, GlProfile::Core).unwrap();
  #[cfg(target_os = "macos")]
  {
    sdl
      .gl_set_attribute(SdlGlAttr::Flags, ContextFlag::ForwardCompatible)
      .unwrap();
  }
```

* The Core profile is a subset of the full features that the spec allows. An
  implementation must provide the Core profile, but it can also provide a
  Compatibility profile, which is the current spec version's features _plus_ all
  the old stuff from previous versions.
* The Forward Compatible flag means that all functions that a particular version
  considers to be "deprecated but available" are instead immediately
  unavailable. It's needed for Mac if you want to have a Core profile. On other
  systems you can have it or not and it doesn't make a big difference. The
  Khronos wiki suggest to only set it if you're on Mac, so that's what I did.

## Make The Window

Finally, once GL is all set, we can make our window.

In some libs you might make the window and then make the GL Context as a
separate step (technically SDL2 lets you do this), but with `beryllium` it just
sticks the window and the GL Context together as a single thing (`glutin` also
works this way, I don't know about `glfw`).

```rust
  let _win = sdl
    .create_gl_window(
      "Hello Window",
      WindowPosition::Centered,
      800,
      600,
      WindowFlags::Shown,
    )
    .expect("couldn't make a window and context");
```

## Processing Events

Once we have a window, we can poll for events. In fact if we _don't_ always poll
for events promptly the OS will usually think that our application has stalled
and tell the user they should kill the program. So we want to always be polling
for those events.

Right now we just wait for a quit event (user clicked the X on the window,
pressed Alt+F4, etc) and then quit when that happens.

```rust
  'main_loop: loop {
    // handle events this frame
    while let Some(event) = sdl.poll_events().and_then(Result::ok) {
      match event {
        Event::Quit(_) => break 'main_loop,
        _ => (),
      }
    }
    // now the events are clear

    // here's where we could change the world state and draw.
  }
}
```

## Done!

That's all there is to it for this lesson. Just a milk run.

* Example Code: [000-basic-window](https://github.com/rust-tutorials/learn-opengl/blob/master/examples/000-basic-window.rs)

## Extras

I'm developing mostly on Windows, and Windows is where most of your market share of users will end up being, so here's some bonus Windows tips:

### Windows Subsystem

I'm going to put the following attribute at the top of the file:

```rust
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
```

This will make is to that a "release" build (with the `--release` flag) will use the "windows" subsystem on Windows, instead of the "console" subsystem. This makes the process not have a console by default, which prevents a little terminal window from running in the background when the program runs on its own. However, we only want that in release mode because we want the ability to print debug message in debug mode.

### Static Linking SDL2

Finally, instead of dynamic linking with SDL2 we could static link with it.

All we have to static link SDL2 instead is change our Cargo.toml file so that instead of saying

```toml
beryllium = "0.2.0-alpha.2"
```

it says

```toml
beryllium = { version = "0.2.0-alpha.1", default-features = false, features = ["link_static"] }
```

However, when we do this, we have to build the SDL2 static lib, which takes longer (about +30 seconds). So I leave it in dynamic link during development because it makes CI go faster.
