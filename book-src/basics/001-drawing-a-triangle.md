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

As a note: you can also bind the value 0 at any time, which clears the vertex
array binding. This might sound a little silly, but it can help spot bugs in
some situations. If you have no VAO bound when you try to call VAO affected
functions it'll generate an error, which usually means that you forgot to bind
the VAO that you really did want to affect.

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

Now that we have a buffer, we can bind it to the binding target that we want.
[`glBindBuffer`](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBindBuffer.xhtml)
takes a target name and a buffer. As you can see on that page, there's a whole
lot of options, but for now we just want to use the `GL_ARRAY_BUFFER` target.

```rust
unsafe {
  glBindBuffer(GL_ARRAY_BUFFER, vbo);
}
```

And, similar to the VAO's binding process, now that our vertex buffer object is
bound to the the `GL_ARRAY_BUFFER` target, all commands using that target will
operate on the buffer that we just made.

(Is this whole binding thing a dumb way to design an API? Yeah, it is. Oh well.)

Now that we have a buffer bound as the `GL_ARRAY_BUFFER`, we can finally use [`glBufferData`](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBufferData.xhtml) to actually send over some data bytes. We have to specify the binding target we want to buffer to, the `isize` of the number of bytes we want to buffer, the const pointer to the start of the data we're buffering, and the usage hint.

Most of that is self explanatory, except the usage hint. Basically there's
memory that's faster or slower for the GPU to use or the CPU to use. If we hint
to the GPU how we intend to use the data and how often we intend to update it
then it has a chance to make a smarter choice of where to put the data. You can
see all the options on the `glBufferData` spec page. For our first demo we want
`GL_STATIC_DRAW`, since we'll just be sending the data once, and then GL will
draw with it many times.

But what data do we send?

#### Demo Vertex Data

We're going to be sending this data:

```rust
type Vertex = [f32; 3];
const VERTICES: [Vertex; 3] =
  [[-0.5, -0.5, 0.0], [0.5, -0.5, 0.0], [0.0, 0.5, 0.0]];
```

It describes a triangle in Normalized Device Context (NDC) coordinates. Each
vertex is an [X, Y, Z] triple, and we have three vertices.

We can also use
[`size_of_val`](https://doc.rust-lang.org/core/mem/fn.size_of_val.html) to get
the byte count, and
[`as_ptr`](https://doc.rust-lang.org/std/primitive.slice.html#method.as_ptr)
followed by
[`cast`](https://doc.rust-lang.org/std/primitive.pointer.html#method.cast) to
get a pointer of the right type. In this case, GL wants a "void pointer", which
isn't a type that exists in Rust, but it's what C calls a "pointer to anything".
Since the buffer function need to be able to accept anything you want to buffer,
it takes a void pointer.

```rust
unsafe {
  glBufferData(
    GL_ARRAY_BUFFER,
    size_of_val(&VERTICES) as isize,
    VERTICES.as_ptr().cast(),
    GL_STATIC_DRAW,
  );
}
```

Good to go!

### Enable A Vertex Attribute

How will the GPU know the correct way to use the bytes we just sent it? Good
question. We describe the "vertex attributes" and then it'll be able to
interpret the bytes correctly.

For each vertex attribute we want to describe we call [`glVertexAttribPointer`](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribPointer.xhtml). There's just one attribute for now (the position of the vertex), so we'll make just one call.

* The `index` is the attribute we're describing. Your selection here has to
  match with the shader program that we make later on. We'll just use 0.
* The `size` is the number of components in the attribute. Since each position
  is a 3D XYZ position, we put 3.
* The `type` is the type of data for the attribute. Since we're using `f32` we
  pass `GL_FLOAT`.
* The `normalized` setting has to do with fixed-point data values. That's not
  related to us right now, so we just leave it as `GL_FALSE`.
* The `stride` is the number of bytes from the start of this attribute in one
  vertex to the start of the same attribute in the next vertex. Since we have
  only one attribute right now, that's just `size_of::<f32>() * 3`. Alternately,
  we can use `size_of::<Vertex>()` and when we edit our type alias at the top
  later on this vertex attribute value will automatically be updated for us.
* The `pointer` value is, a little confusingly, not a pointer to anywhere in
  _our_ memory space. Instead, it's a pointer to the start of this vertex
  attribute within the buffer _as if_ the buffer itself were starting at memory
  location 0. Little strange, but whatever. Since this attribute is at the start
  of the vertex, we use 0. When we have more attributes later all the attributes
  will usually end up with the same `stride` but different `pointer` values. I'll be sure to review this point again later, because it's a little weird.

Once we've described the vertex attribute pointer, we also need to enable it
with
[`glEnableVertexAttribArray`](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glEnableVertexAttribArray.xhtml).
It just takes the name of the `index` to enable, so we pass 0.

Also, when we provide the stride it wants `isize` and Rust always uses `usize`
for sizes, so we have to convert there. In this case we'll use the
[`TryInto::try_into`](https://doc.rust-lang.org/core/convert/trait.TryInto.html)
trait method, along with an `unwrap`. It should work, but if somehow it would
have overflowed, it's better to explode in a controlled manner _now_ than cause
the GPU to read memory way out of bounds at some unknown point _later_.

Also also, we have to convert the pointer location using `usize` values _and
then_ cast to a const pointer once we have our `usize`. We **do not** want to
make a null pointer and then offset it with the `offset` method. That's gonna
generate an out of bounds pointer, which is UB. We could try to remember to use
the `wrapping_offset` method, or we could just do all the math in `usize` and
then cast at the end. I sure know which one I prefer.

```rust
unsafe {
  glVertexAttribPointer(
    0,
    3,
    GL_FLOAT,
    GL_FALSE,
    size_of::<Vertex>().try_into().unwrap(),
    0 as *const _,
  );
  glEnableVertexAttribArray(0);
}
```

## Send A Program

Okay, we have some bytes sent to the GPU, and the GPU knows that it's a series
of vertexes which are each three `f32` values. How does it know what to do from
there? Again, with these good questions.

When your GPU draws a picture, that's called the "graphics pipeline". Some parts
of the pipeline are totally fixed, or you can pick from one of a few options.
The rest is done by a "shader program".

We need to make a [Program
Object](https://www.khronos.org/opengl/wiki/GLSL_Object#Program_objects),
compile and attach some shader stages to it, link the stages together, and then
use that program.

Of course, to attach those compiled shader stages we need to make some [Shader
Objects](https://www.khronos.org/opengl/wiki/GLSL_Object#Shader_objects) too.
It's objects all the way down!

### Create A Vertex Shader

First we want a [Vertex Shader](https://www.khronos.org/opengl/wiki/Vertex_Shader).

This time we're _not_ calling a "gen" style method with an array to fill and
getting a huge array of new shaders. GL assumes that you'll use sufficiently few
shaders that you can make them one at a time, so we call
[`glCreateShader`](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCreateShader.xhtml) with a shader type and we get just one shader back. Or 0 if there was an error.

If you look at the spec page there (and you should naturally have at least a
quick look at _all_ of the spec pages I'm linking for you!), then you'll see
that there's a lot of types of shader! We only actually need _two_ of them to
get our program going. Actually most GL programs will just use the Vertex and
Fragment shader. Even like complete products that aren't just demos. Vertex and
Fragment are essential, the others are optional and specialized.

One vertex shader please.

```rust
unsafe {
  let vertex_shader = glCreateShader(GL_VERTEX_SHADER);
  assert_ne!(vertex_shader, 0);
}
```

Thank you.

Now we need to upload some source code for this shader. The source code needs to
be written in a language called
[GLSL](https://www.khronos.org/opengl/wiki/Core_Language_(GLSL)). Let's go with
a vertex shader that's about as simple as you can possibly get with a vertex
shader:

```rust
const VERT_SHADER: &str = r#"#version 330 core
  layout (location = 0) in vec3 pos;
  void main() {
    gl_Position = vec4(pos.x, pos.y, pos.z, 1.0);
  }
"#;
```

That's one long string literal with a lot of stuff inside it.

#### Inspecting The Vertex Source

The first line of the vertex shader is a `#version 330 core`. You have to have
this line on the very first line, it identifies the version of the GLSL language
that your program is written for. In the same way that each version of OpenGL
adds a little more stuff you can do, each version of GLSL has a little more you
can do too. Version 330 is the version that goes with OpenGL 3.3, and we're using the core profile.

Now we get to the actual interesting bits. The job of the vertex shader is to
read in the vertex attribute values from the buffer, do whatever, and then write
to `gl_Position` with the position that this vertex should end up at.

```glsl
layout (location = 0) in vec3 pos;
```

This specifies that at attribute index 0 within the buffer (remember how we set
vertex attribute 0 before?) there's an `in` variable, of type `vec3`, which
we're going to call `pos`.

```glsl
void main() {
  gl_Position = vec4(pos.x, pos.y, pos.z, 1.0);
}
```

Like with Rust and C, GLSL programs start at `main`. Our main function reads the
`x`, `y`, and `z` of the vertex position, and then sticks a `1.0` on the end,
and writes that `vec4` into the `gl_Position` variable. It just copies over the
data, no math or anything. Not the most exciting. We'll have plenty of math
later, don't worry.

#### Upload The Vertex Shader Source, and Compile

Now that we've got some source, we need to send it over. For this we use
[`glShaderSource`](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glShaderSource.xhtml),
which is a little tricky to get right the first time. The first argument is the
name of a shader to set the source for. Next we have to describe the string data
sorta like with `glBufferData`, but the format is a little wonky. They're
expecting a length of two different arrays, and the first array is full of
string data, while the second array is full of the lengths of each string. This is supposed to allow you to... I dunno. It's some sort of C nonsense.

What we do in Rust is this:

```rust
unsafe {
  glShaderSource(
    vertex_shader,
    1,
    &(VERT_SHADER.as_bytes().as_ptr().cast()),
    &(VERT_SHADER.len().try_into().unwrap()),
  );
}
```

Ah, look a little weird? Yeah it's still a little weird. So what's happening is that first we're saying that out array of strings and our array of string lengths will both have length 1. Like with `glGenBuffer`.

Then we're passing a pointer _to the pointer_ of the start of the string. So we write `&(expr)`, with a `&` forced to the outside of our expression by the parentheses. If you don't have those parentheses then the order of operations goes wrong: it takes a reference to `VERTEX_SHADER`, calls `as_bytes` on that, and then you get a very wrong value at the end.

Then, for the length we do basically the same thing. We take a pointer _to the length_ after getting the string length as an `i32` value.

Once that string data is uploaded we call [`glCompileShader`](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCompileShader.xhtml) to tell GL to compile it, and we're home free.

```rust
unsafe {
  glCompileShader(vertex_shader);
}
```

#### Check For An Error

I lied just now, we're not home free.

Obviously, the one thing I'm very sure that you know about programming, is that sometimes when you compile a program there's an error. Maybe you spelled a word wrong, maybe a type didn't match, whatever. Anything could go wrong, so we have to check for that.

The checking process is actually more annoying than the compilation!

First we use
[`glGetShaderiv`](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetShader.xhtml).
The `iv` part means "int" "vector", so the output value will be that they'll
write to a pointer we send them. We have to pass the name of the shader we want
info on, the `GL_COMPILE_STATUS` specifier to get the compile status, and a
pointer that they can write to so we can get a value back. Side note:
out-parameter pointers are terrible, please never design your API this way.

```rust
unsafe {
  let mut success = 0;
  glGetShaderiv(vertex_shader, GL_COMPILE_STATUS, &mut success);
}
```

So this `success` value is bool-style 1 for yes and 0 for no. You can also use
`GL_TRUE` and `GL_FALSE` but the types won't match up and in C you don't get
automatic conversion, so we'll just check for 0 (no success).

If there was not a success, then then _real_ fun begins. That means we have to get a message out of the shader log.

We _could_ check the info log length with `GL_INFO_LOG_LENGTH`, then allocate a
perfectly sized buffer and have them write to the buffer. However, that gives us
a `Vec<u8>` (or `Vec<c_char>` if you want), and then we convert that to
`String`. I like to use `String::from_utf8_lossy` when I've got unknown bytes,
which allocates its own buffer anyway, so we'll just allocate 1k of `Vec` and assume that the log length is 1024 or less.

So we call
[`glGetShaderInfoLog`](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetShaderInfoLog.xhtml),
with the shader we want the info log for, the maximum capacity of our buffer, a
pointer to the spot where it will store the number of bytes written, and the
pointer to the buffer of course. Then we set the length of the `Vec`, convert to
`String`, and `panic!` (at the disco) with that error message.

```rust
unsafe {
  if success == 0 {
    let mut v: Vec<u8> = Vec::with_capacity(1024);
    let mut log_len = 0_i32;
    glGetShaderInfoLog(
      vertex_shader,
      1024,
      &mut log_len,
      v.as_mut_ptr().cast(),
    );
    v.set_len(log_len.try_into().unwrap());
    panic!("Vertex Compile Error: {}", String::from_utf8_lossy(&v));
  }
}
```

### Create A Fragment Shader

Making a [Fragment Shader](https://www.khronos.org/opengl/wiki/Fragment_Shader)
is nearly identical to making a vertex shader, except we pass a different shader
type. Also, we have some different source code of course.

```rust
unsafe {
  let fragment_shader = glCreateShader(GL_FRAGMENT_SHADER);
  assert_ne!(fragment_shader, 0);
}
```

And the fragment source looks like this

```rust
const FRAG_SHADER: &str = r#"#version 330 core
  out vec4 final_color;

  void main() {
    final_color = vec4(1.0, 0.5, 0.2, 1.0);
  }
"#;
```

#### Inspecting The Fragment Source

Again we have a version line, always nice to have versions.

```glsl
out vec4 final_color;
```

This says that we're going to output a `vec4`, and we'll call it `final_color`.
With the `gl_Position` value in the vertex shader, it's just assumed to be there
since every vertex shader needs to write a position out. With fragment shaders,
the system will just assume that whatever `vec4` your fragment shader puts out,
with any name, is the output color.

```glsl
void main() {
  final_color = vec4(1.0, 0.5, 0.2, 1.0);
}
```

Here, the color is a kind of orange color, and it's the same everywhere.
Anywhere we have a fragment, we'll have an orange pixel.

I assure you that both vertex and fragment shaders will become more complex as
we go, but if you just want to draw _anything_ it's this simple.

#### Upload The Fragment Shader Source

And we upload and compile like before:

```rust
unsafe {
  glShaderSource(
    fragment_shader,
    1,
    &(FRAG_SHADER.as_bytes().as_ptr().cast()),
    &(FRAG_SHADER.len().try_into().unwrap()),
  );
  glCompileShader(fragment_shader);
}
```

#### Check For An Error, Again

And we check for an error like before:

```rust
unsafe {
  let mut success = 0;
  glGetShaderiv(fragment_shader, GL_COMPILE_STATUS, &mut success);
  if success == 0 {
    let mut v: Vec<u8> = Vec::with_capacity(1024);
    let mut log_len = 0_i32;
    glGetShaderInfoLog(
      fragment_shader,
      1024,
      &mut log_len,
      v.as_mut_ptr().cast(),
    );
    v.set_len(log_len.try_into().unwrap());
    panic!("Fragment Compile Error: {}", String::from_utf8_lossy(&v));
  }
}
```

This is all a very good candidate for wrapping into an easier to use function,
but we'll get to that after we can at least see a triangle.

### Create A Program

A program combines several shader "stages" such as vertex and fragment, and lets you have a completed graphics pipeline.

We use [`glCreateProgram`](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCreateProgram.xhtml) to create one, and then we use [`glAttachShader`](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glAttachShader.xhtml) to connect both shaders we have so far. Finally we call [`glLinkProgram`](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glLinkProgram.xhtml) to connect the shader stages into a single, usable whole.

```rust
unsafe {
  let shader_program = glCreateProgram();
  glAttachShader(shader_program, vertex_shader);
  glAttachShader(shader_program, fragment_shader);
  glLinkProgram(shader_program);
}
```

And we have to check the `GL_LINK_STATUS` with [`glGetProgramiv`](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetProgram.xhtml), and grab the link error log if there was a link error.

```rust
unsafe {
  let mut success = 0;
  glGetProgramiv(shader_program, GL_LINK_STATUS, &mut success);
  if success == 0 {
    let mut v: Vec<u8> = Vec::with_capacity(1024);
    let mut log_len = 0_i32;
    glGetProgramInfoLog(
      shader_program,
      1024,
      &mut log_len,
      v.as_mut_ptr().cast(),
    );
    v.set_len(log_len.try_into().unwrap());
    panic!("Program Link Error: {}", String::from_utf8_lossy(&v));
  }
}
```

Finally, and this part _is_ a little weird, we can mark the shaders to be deleted with [`glDeleteShader`](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDeleteShader.xhtml). They won't _actually_ get deleted until they're unattached from the program we have, but we can call delete now and worry about one less thing later on.

```rust
unsafe {
  glDeleteShader(vertex_shader);
  glDeleteShader(fragment_shader);
}
```

Finally, after all that, we can call
[`glUseProgram`](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUseProgram.xhtml)
to set our program as the one to use during drawing.

## Vsync

Last thing before we move on to the main loop, let's turn on
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

## Clear The Screen

In the main loop, after we process our events, we start our drawing with a call
to
[`glClear`](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glClear.xhtml).
In this case we specify the `GL_COLOR_BUFFER_BIT`, since we want to clear the
color values. You could clear the other bits too, but since we're not using them right now we'll just clear the colors.

```rust
unsafe {
  glClear(GL_COLOR_BUFFER_BIT);
}
```

## Draw The Triangle

To actually draw our triangle we call [`glDrawArrays`](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDrawArrays.xhtml).

* The `mode` is how to connect the vertexes together. We use `GL_TRIANGLES`
  which makes it process the vertexes in batches of 3 units each into however
  many triangles that gets you.
* The `first` value is the first vertex index to use within our vertex buffer
  data. Since we want to draw all three of our vertexes, we start at index 0.
* The `count` value it the number of indices to be drawn. Since we want to draw
  all three of our vertexes, we use 3.

```rust
unsafe {
  glDrawArrays(GL_TRIANGLES, 0, 3);
}
```

Be _extra careful_ with this call. If you tell it to draw too many triangles the
GPU will run right off the end of the array and segfault the program.

## Swap The Window Buffers

Once the drawing is done, we have to swap the window's draw buffer and display
buffer, with `swap_window`. This will make the picture we just drew actually be
displayed to the user. With vsync on it'll also block until the image is actually displayed.

```rust
win.swap_window();
```

## Done!

* Example Code: [001-triangle-arrays1](https://github.com/rust-tutorials/learn-opengl/blob/master/examples/001-triangle-arrays1.rs)
