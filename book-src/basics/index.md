# Basics

TODO: maybe some of this can go into the Introduction? whatever.

We'll be using OpenGL 3.3. The latest version is 4.6, but we'll still be using
3.3. The main reason for this is because if we take a quick look at Mac's
[supported OpenGL versions](https://support.apple.com/en-us/HT202823) we can see
that they support 3.3 on older stuff and 4.1 on newer stuff. Macs don't get
OpenGL 4.6 like Windows and Linux have. Oh well. Feel free to use this book and
then learn the stuff that got added after 3.3 if you don't care about supporting
old macs.

OpenGL is a _specification_. Not a specific implementation, just a
specification. Each graphics card has its own driver, which has its own
implementation of OpenGL. This means that you can run in to bugs on one card
that doesn't show up on other cards. Fun!

OpenGL is specified in terms of a series of C functions that you can call. They
all affect a "Context". A GL context has all sorts of state inside. There's GL
calls to draw things, but there's also a lot of calls to carefully set the state
before the drawing happens. Both types of call are equally important to getting
a picture on the screen.

So we'll be doing a _lot_ of FFI calls. FFI calls are naturally `unsafe`,
because the Rust compiler can't see what's going on over there.

If you don't want to have to call `unsafe` code you can try
[luminance](https://github.com/rust-tutorials/learn-luminance), or
[glium](https://docs.rs/glium), or [wgpu](https://docs.rs/wgpu) or, something
like that. You don't _have_ to call `unsafe` code to get a picture on the
screen.

But if you want to know how people _built_ those other libraries that let you do
those cool things, you gotta learn this direct usage stuff.

## Prior Knowledge

You should generally be familiar with all the topics covered in [The Rust
Programming Language](https://doc.rust-lang.org/book/), but you don't need to
have them memorized. You can look things up again if you need to.

I usually tell folks that they should read [The
Rustonomicon](https://doc.rust-lang.org/nomicon/) before doing a lot of unsafe
code. However, with GL you're not really doing a lot of hackery _within Rust_
that could go wrong. It's just that the driver could explode in your face if you
look at it funny. Or even if you don't, because drivers are just buggy
sometimes. Oh well, that's life.

## Libraries Used

As I start this project, this is what my Cargo.toml looks like.

```toml
[dependencies]
bytemuck = "1"
ogl33 = { version = "0.2.0", features = ["debug_error_checks"]}

[dev-dependencies]
beryllium = "0.2.0-alpha.4"
imagine = "0.0.5"
```

So the library itself, where we'll put our useful GL helpers, will depend on

* [ogl33](https://docs.rs/ogl33), which gives us bindings to OpenGL.
  * It's similar to the [gl](https://docs.rs/gl) crate (which loads OpenGL 4.6),
    but all functions and constants use their real names exactly as you'd see in
    C code. It makes it a lot easier to read books and blogs about OpenGL that
    are written for C (which is essentially all of them), and then quickly
    translate it to Rust.
* [bytemuck](https://docs.rs/bytemuck), which is a handy crate for casting
  around plain data types.

And then if you're not familiar with "dev-dependencies", that's bonus
dependencies that tests and examples can use (but not bins!). Since our example
programs will be examples in the `examples/` directory, they'll be able to use
"dev-dependencies" without that affecting the lib itself. That way if someone
else wants to use the lib they can use _just_ the lib in their own program,
without having to also build the stuff we're using for our examples.

* [beryllium](https://docs.rs/beryllium) is an SDL2 wrapper. It will dynamic
  link by default so you'll need `SDL2.dll` in your path to run a program. You
  can swap this to static linking, I describe that at the end of the first
  lesson.
  * Other options: [glutin](https://docs.rs/glutin), [glfw](https://docs.rs/glfw)
* [imagine](https://docs.rs/imagine) is a PNG parser (not used right away, but
  soon enough).
  * Other options: [image](https://docs.rs/image), which uses
    [png](https://docs.rs/png)
* [ultraviolet](https://docs.rs/ultraviolet) is a graphics linear algebra crate.
  * Other options: [glam](https://docs.rs/glam),
    [nalgebra-glm](https://docs.rs/nalgebra-glm)

Full disclosure: I wrote almost all of the crates on the list. Other than
`ultraviolet`, which was done by [Fusha](https://github.com/termhn), because I'm
a dummy who can't do math.

However, I'm writing the book, so I get to use my own crates while I do it. I
think this is fair, and I'm also providing alternative suggestions for each one,
so I don't feel bad about it.
