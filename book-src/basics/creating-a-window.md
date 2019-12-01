# Creating A Window

So, this part of the tutorial is very library specific, so I won't focus on it
too much.

Basically, we have to open a window, and we also need a GL context to go with that window.

On most platforms, you have to specify that you'll be using GL _before_ you create the window, so that the window itself can be created with the correct settings to support GL once it's made.

In our case, first we turn on SDL itself:

```rust
use beryllium::*;

fn main() {
  let sdl = SDL::init(InitFlags::Everything).expect("couldn't start SDL");
```

Then we set some GL attributes that we want to use:

```rust
  sdl.gl_set_attribute(SdlGlAttr::MajorVersion, 3).unwrap();
  sdl.gl_set_attribute(SdlGlAttr::MinorVersion, 3).unwrap();
  sdl.gl_set_attribute(SdlGlAttr::Profile, GlProfile::Core).unwrap();
  sdl
    .gl_set_attribute(SdlGlAttr::Flags, ContextFlag::ForwardCompatible)
    .unwrap();
```

Finally we can make our window.

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

Once we have a window, we can poll for events. Right now we just wait for a quit
event (user clicked the X on the window, pressed Alt+F4, etc) and then quit when
that happens.

```rust
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
```

That's all there is to it. Just a milk run.

## Extras

I'm developing mostly on Windows, and Windows is where most of your market share of users will end up being, so here's some bonus Windows tips:

### Windows Subsystem

I'm going to put the following attribute at the top of the file:

```rust
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
```

This will make is to that a "release" build (with the `--release` flag) will use the "windows" subsystem on Windows, instead of the "console" subsystem. This makes the process not have a console by default, which prevents a little terminal window from running in the background when the program runs on its own. However, we only want that in release mode because we want the ability to print debug message in debug mode.

### Static C Runtime

Also, I'm going to add a `.cargo/` folder to the project and put a `config` file inside.

```toml
[build]
rustflags = ["-C","target-feature=+crt-static"]
```

This will make Rust compile in a static C runtime when it builds the binaries, so that the binaries can be sent to others without them needing to have the MSVC redistributable DLLs or other files like that on their machine. It's not on by default because I don't even know why. I think it makes programs marginally larger, but it doesn't seem to make them compile slower so whatever.

We could instead make a totally `no_std` program if we wanted to, but that's a whole set of steps and not really OpenGL related at all, so for this tutorial book we'll use the "quick and dirty" way to get our programs to be easily portable.

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
