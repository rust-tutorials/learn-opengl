# Basics

TODO: maybe some of this can go into the Introduction? whatever.

It's become the "older" style among the graphics APIs, with Vulkan / DX12 /
Metal being the new hotness.

However, I think that it's easy enough to use and you don't often need the final
ounce of performance that you can get with Vulkan.

We'll be using OpenGL 3.3. The latest version is 4.6, but we'll be using 3.3.
The main reason for this is because if we take a quick look at Mac's [supported
OpenGL versions](https://support.apple.com/en-us/HT202823) we can see that they
support 3.3 on old stuff and 4.1 on new stuff. Macs don't get OpenGL 4.6 like
Windows and Linux have. Oh well. Feel free to use this book and then learn the
stuff that got added after 3.3 if you don't care about supporting old macs.

OpenGL is a _specification_, and it's a specification for a C API. It's probably
implemented in C as well, but actually you could implement it in any language
that can expose a C API. Time to rewrite OGL in Rust, ne?

So we'll be doing a lot of FFI calls. FFI calls are naturally `unsafe`. If
you're not comfortable with that then go use [glium](https://docs.rs/glium), or
[wgpu](https://docs.rs/wgpu) or something. Phaazon converted their [luminance
tutorial](https://github.com/rust-tutorials/learn-luminance) into a more bookish
form recently too. You don't _have_ to use OpenGL directly.

But if you want to know how people _built_ those other libraries, you gotta
learn this direct usage stuff.

## Prior Knowledge

You should generally be familiar with all the topics covered in [The Rust
Programming Language](https://doc.rust-lang.org/book/), but you don't need to
have them memorized you can look things up again if you need to.

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
ogl33 = { version = "0.1", features = ["debug_error_checks", "debug_trace_messages"]}

[dev-dependencies]
beryllium = "0.2.0-alpha.2"
imagine = "0.0.2"
ultraviolet = "0.3"
```

So the library itself, where we'll put our useful GL helpers, will depend on

* [ogl33](https://docs.rs/ogl33), which gives us bindings to OpenGL.
  * It works mostly like the [gl](https://docs.rs/gl) crate, except it _only_
    connects to OpenGL 3.3 (the `gl` crate builds for 4.6).
* [bytemuck](https://docs.rs/bytemuck), which is a handy crate for casting around data types.

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
  * Other options: [winit](https://docs.rs/winit), [glfw](https://docs.rs/glfw)
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