# Drawing A Triangle

In this lesson, we'll do a lot of setup just to be able to draw a single
triangle.

Don't worry, once you do the first batch of setup, drawing that _second_
triangle is easy.

## Load The Opengl Functions

Unlike most libraries that you can use in a program, OpenGL cannot be statically
linked to. Well, you can static link to very old versions, but any sort of newer
OpenGL library is installed on the user's system as a dynamic library that you
load at runtime. This way the user can get their video driver updates and then your program just loads in the new driver file the next time it turns on.

The details aren't too important to the rest of what we want to do, so I won't
discuss it here. Perhaps an appendix page or something at some point in the
future. The `ogl33` crate handles it for us. As a reminder, you could also use
the `gl` or `glow` crates.

After we open the window, we just say that we want to load up every OpenGL
function.

```rust
unsafe {
  load_gl_with(|f_name| win.get_proc_address(f_name));
}
```

## Set The "Clear" Screen Color

When we clear the previous image's data at the start of our drawing, by default it would clear to black. Since we'll only have one thing at a time to draw for a little bit, let's use a slightly softer sort of color.

We just need to call
[`glClearColor`](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glClearColor.xhtml)
with the red, green, blue, and alpha intensities that we want to use.

```rust
unsafe {
  glClearColor(0.2, 0.3, 0.3, 1.0);
}
```

This is a blue-green sort of color that's only a little bit away from being
gray. You can _kinda_ tell that even before we open the window. The channel
values are all close (which is gray), but there's a little less red, so it tilts
towards being a blue-green.

The alpha value isn't important for now because our window itself isn't
transparent (so you can't see pixels behind it) and we're not doing any color
blending yet (so the alpha of the clear color compared to some other color
doesn't come into play). Eventually it might matter, so we'll just leave it on
"fully opaque" for now.

## Send A Triangle

At this point there's two main actions we need to take before we're ready for our triangle to be drawn.

* We need to get some triangle data to the video card in a way it understands.
* We need to get a program to the video card so that it can make use of the data.

Neither task depends on the other, so we'll send our triangle data first and
then send our program.

### Generate A Vertex Array Object

A [Vertex Array
Object](https://www.khronos.org/opengl/wiki/Vertex_Specification#Vertex_Array_Object)
(VAO) is an object that collects together a few different bits of stuff.
Basically, at any given moment there either is a Vertex Array Object "bound",
meaning it's the active one, or there is not one bound, which makes basically
all commands that relate to buffering data and describing data invalid. Since we
want to buffer some data and describe it, we need to have a VAO bound.

You make a vertex array object with
[`glGenVertexArrays`](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGenVertexArrays.xhtml).
It takes the length of an array to fill, and a pointer to the start of that
array. Then it fills the array with the names of a bunch of new VAOs. You're
allowed to make a lot of vertex arrays at once if you want, but we just need one
for now. Luckily, a pointer to just one thing is the same as a pointer to an
array of length 1.

Also, `glGenVertexArrays` _shouldn't_ ever return 0, but if some sort of bug
happened it could, so we'll throw in a little assert just to check that.

```rust
unsafe {
  let mut vao = 0;
  glGenVertexArrays(1, &mut vao);
  assert_ne!(vao, 0);
}
```

Once we have a VAO we can "bind" it with
[`glBindVertexArray`](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBindVertexArray.xhtml)
to make it the active VAO. This is a _context wide_ effect, so now all GL
functions in our GL context will do whatever they do with this VAO as the VAO to
work with.

### Generate A Vertex Buffer Object

To actually get some bytes of data to the video card we need a [Vertex Buffer
Object](https://www.khronos.org/opengl/wiki/Vertex_Specification#Vertex_Buffer_Object)
(VBO) to go with our Vertex Array Object. You might get sick of the words
"vertex" and "object" by the time you've read this whole book.

This time things are a little different than with the VAO. Instead of calling a
function to make and bind specifically a _vertex_ buffer object, there's just a
common function to make and bind buffers of all sorts. It's called
[`glGenBuffers`](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGenBuffers.xhtml),
and it works mostly the same as `glGenVertexArrays` did, you pass a length and a
pointer and it fills an array.

```rust
unsafe {
  let mut vbo = 0;
  glGenBuffers(1, &mut vbo);
  assert_ne!(vbo, 0);
}
```

Now that we have a buffer, we can bind it to the binding target that we want. [`glBindBuffer`](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBindBuffer.xhtml) takes a target name and a buffer. As you can see on that page, there's a whole lot of options, but for now we just want `GL_ARRAY_BUFFER`.

```rust
unsafe {
  glBindBuffer(GL_ARRAY_BUFFER, vbo);
}
```

## Vsync

Finally, let's turn on
[vsync](https://en.wikipedia.org/wiki/Screen_tearing#Vertical_synchronization),
which will make our `swap_window` call block the program until the image has
actually been presented to the user. This makes the whole program run no faster
than the screen's refresh rate, usually at least 60fps (sometimes more these
days). This is usually a good thing. We can't show them images faster than the
screen will present them anyway, so we can let the CPU cool down a bit, maybe
save the battery even if they're on a laptop.

```rust
// this goes any time after window creation.
win.set_swap_interval(SwapInterval::Vsync);
```

## Done!

* Example Code: [001-triangle-arrays1](https://github.com/rust-tutorials/learn-opengl/blob/master/examples/001-triangle-arrays1.rs)

## Cleanup

* Example Code: [002-triangle-arrays2](https://github.com/rust-tutorials/learn-opengl/blob/master/examples/002-triangle-arrays2.rs)
