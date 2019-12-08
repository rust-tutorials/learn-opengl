# Triangle Cleanup

Now that we can see the basics of what's going on we're going to do a bit of
clean up. This won't change what we're drawing, it'll just help us sort out the
easy stuff (which we can mark safe and then worry about a lot less) from the
unsafe stuff (which we will always have to pay attention to).

From here on, the examples will all have

```rust
use learn_opengl as learn;
```

We'll use our helpers via `learn::func_name()`. You could of course import the
functions and then leave off the prefix, but in tutorial code you always want to
aim for a little more clarity than is strictly necessary.

## First, A Note On Using `glGetError`

The `ogl33` crate will _automatically_ call
[`glGetError`](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetError.xhtml)
after each GL call if the `debug_error_checks` is enabled along with
`debug_assertions`. This means that we don't have to call `glGetError` ourselves
to see any errors get reported when we're testing the program. However, if we
wanted to check errors without `debug_assertions` on then we'd have to call
`glGetError` manually. Or if you were using a crate to load and call GL other
than `ogl33` I guess.

The way that `glGetError` works is pretty simple: You call it, and you get a
value back. If there's no pending errors you get `GL_NO_ERROR`, if there's a
pending error you get some other value. However, depending on driver there might
be more than one error pending at once. So you should call `glGetError` _until_
you finally get a `GL_NO_ERROR`.

## Setting The Clear Color

Making `glClearColor` safe is easy, there's nothing that can go wrong:

```rust
/// Sets the color to clear to when clearing the screen.
pub fn clear_color(r: f32, g: f32, b: f32, a: f32) {
  unsafe { glClearColor(r, g, b, a) }
}
```

and then in the example we'd call it like this:

```rust
learn::clear_color(0.2, 0.3, 0.3, 1.0);
```

## Vertex Array Objects

With the Vertex Array Object stuff, we're just wrapping the name in our own type
and then giving methods for the operations that go with it. However, we don't
yet know all of the functions that we might need to use, so we'll keep the inner
value public and we can just pull that out at any time if we need to.

We'll want a way to make them, and to bind them.

```rust
/// Basic wrapper for a [Vertex Array
/// Object](https://www.khronos.org/opengl/wiki/Vertex_Specification#Vertex_Array_Object).
pub struct VertexArray(pub GLuint);
impl VertexArray {
  /// Creates a new vertex array object
  pub fn new() -> Option<Self> {
    let mut vao = 0;
    unsafe { glGenVertexArrays(1, &mut vao) };
    if vao != 0 {
      Some(Self(vao))
    } else {
      None
    }
  }

  /// Bind this vertex array as the current vertex array object
  pub fn bind(&self) {
    unsafe { glBindVertexArray(self.0) }
  }

  /// Clear the current vertex array object binding.
  pub fn clear_binding() {
    unsafe { glBindVertexArray(0) }
  }
}
```

Then we use it like this:

```rust
let vao = VertexArray::new().expect("Couldn't make a VAO");
vao.bind();
```

## Buffers

For buffers it's a little more tricky because we have to make sure that we don't
design to heavily for just Vertex Buffers and block ourselves from easily using
other types of buffers. In fact since we'll want to use ElementArray buffers in
the next lesson we can add that now to a `BufferType`.

```rust
/// The types of buffer object that you can have.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BufferType {
  /// Array Buffers holds arrays of vertex data for drawing.
  Array = GL_ARRAY_BUFFER as isize,
  /// Element Array Buffers hold indexes of what vertexes to use for drawing.
  ElementArray = GL_ELEMENT_ARRAY_BUFFER as isize,
}
```

Then the buffers themselves will accept a BufferType argument when we bind.

```rust
/// Basic wrapper for a [Buffer
/// Object](https://www.khronos.org/opengl/wiki/Buffer_Object).
pub struct Buffer(pub GLuint);
impl Buffer {
  /// Makes a new vertex buffer
  pub fn new() -> Option<Self> {
    let mut vbo = 0;
    unsafe {
      glGenBuffers(1, &mut vbo);
    }
    if vbo != 0 {
      Some(Self(vbo))
    } else {
      None
    }
  }

  /// Bind this vertex buffer for the given type
  pub fn bind(&self, ty: BufferType) {
    unsafe { glBindBuffer(ty as GLenum, self.0) }
  }

  /// Clear the current vertex buffer binding for the given type.
  pub fn clear_binding(ty: BufferType) {
    unsafe { glBindBuffer(ty as GLenum, 0) }
  }
}
```

Finally, to buffer some data, we'll leave that as a free function. It'll take a
buffer type and a slice of bytes, and a usage. I don't think we really need to
make a special enum for usage values, so we'll just keep using `GLenum` for
the usage argument.

```rust
/// Places a slice of data into a previously-bound buffer.
pub fn buffer_data(ty: BufferType, data: &[u8], usage: GLenum) {
  unsafe {
    glBufferData(
      ty as GLenum,
      data.len().try_into().unwrap(),
      data.as_ptr().cast(),
      usage,
    );
  }
}
```

And the usage code looks like this:

```rust
let vbo = Buffer::new().expect("Couldn't make a VBO");
vbo.bind(BufferType::Array);
learn::buffer_data(
  BufferType::Array,
  bytemuck::cast_slice(&VERTICES),
  GL_STATIC_DRAW,
);
```

The [`bytemuck`](https://docs.rs/bytemuck) crate is a handy crate for safe casting operations. In this case, it's letting us cast our `&[[f32;3]]` into `&[u8]`.

## Vertex Attribute Pointers

This stuff is wild!

It's actually really hard to come up with a general vertex attribute pointer
system that works with arbitrary rust data type inputs and also always lines up
with the shaders you're using... so I'm _not even going to bother_.

It's okay to have a few unsafe parts where you just always pay attention to what
you're doing.

## Shaders and Programs

So obviously we want a shader type enum:

```rust
/// The types of shader object.
pub enum ShaderType {
  /// Vertex shaders determine the position of geometry within the screen.
  Vertex = GL_VERTEX_SHADER as isize,
  /// Fragment shaders determine the color output of geometry.
  ///
  /// Also other values, but mostly color.
  Fragment = GL_FRAGMENT_SHADER as isize,
}
```

And then... well what we really want is to say to our library: "I have this string and it's a shader of this type, just make it happen".

```rust
/// A handle to a [Shader
/// Object](https://www.khronos.org/opengl/wiki/GLSL_Object#Shader_objects)
pub struct Shader(pub GLuint);
impl Shader {
  pub fn from_source(ty: ShaderType, source: &str) -> Result<Self, String> {
    unimplemented!()
  }
}
```

Like that's the final interface we want to have, right? But to support that
operation we probably want to make each individual operation a little easier to
use. That way we can think about the bigger operation in terms of easy to use
smaller operations. Sometimes having too many middle layers can hide a detail
that you don't want hidden, but this is just a little extra in the middle so
it's fine.

I'm just gonna throw it all down because you've seen it before and there's not
much new to comment on.

```rust
impl Shader {
  /// Makes a new shader.
  ///
  /// Prefer the [`Shader::from_source`](Shader::from_source) method.
  ///
  /// Possibly skip the direct creation of the shader object and use
  /// [`ShaderProgram::from_vert_frag`](ShaderProgram::from_vert_frag).
  pub fn new(ty: ShaderType) -> Option<Self> {
    let shader = unsafe { glCreateShader(ty as GLenum) };
    if shader != 0 {
      Some(Self(shader))
    } else {
      None
    }
  }

  /// Assigns a source string to the shader.
  ///
  /// Replaces any previously assigned source.
  pub fn set_source(&self, src: &str) {
    unsafe {
      glShaderSource(
        self.0,
        1,
        &(src.as_bytes().as_ptr().cast()),
        &(src.len().try_into().unwrap()),
      );
    }
  }

  /// Compiles the shader based on the current source.
  pub fn compile(&self) {
    unsafe { glCompileShader(self.0) };
  }

  /// Checks if the last compile was successful or not.
  pub fn compile_success(&self) -> bool {
    let mut compiled = 0;
    unsafe { glGetShaderiv(self.0, GL_COMPILE_STATUS, &mut compiled) };
    compiled == i32::from(GL_TRUE)
  }

  /// Gets the info log for the shader.
  ///
  /// Usually you use this to get the compilation log when a compile failed.
  pub fn info_log(&self) -> String {
    let mut needed_len = 0;
    unsafe { glGetShaderiv(self.0, GL_INFO_LOG_LENGTH, &mut needed_len) };
    let mut v: Vec<u8> = Vec::with_capacity(needed_len.try_into().unwrap());
    let mut len_written = 0_i32;
    unsafe {
      glGetShaderInfoLog(
        self.0,
        v.capacity().try_into().unwrap(),
        &mut len_written,
        v.as_mut_ptr().cast(),
      );
      v.set_len(len_written.try_into().unwrap());
    }
    String::from_utf8_lossy(&v).into_owned()
  }

  /// Marks a shader for deletion.
  ///
  /// Note: This _does not_ immediately delete the shader. It only marks it for
  /// deletion. If the shader has been previously attached to a program then the
  /// shader will stay allocated until it's unattached from that program.
  pub fn delete(self) {
    unsafe { glDeleteShader(self.0) };
  }

  /// Takes a shader type and source string and produces either the compiled
  /// shader or an error message.
  ///
  /// Prefer [`ShaderProgram::from_vert_frag`](ShaderProgram::from_vert_frag),
  /// it makes a complete program from the vertex and fragment sources all at
  /// once.
  pub fn from_source(ty: ShaderType, source: &str) -> Result<Self, String> {
    let id = Self::new(ty)
      .ok_or_else(|| "Couldn't allocate new shader".to_string())?;
    id.set_source(source);
    id.compile();
    if id.compile_success() {
      Ok(id)
    } else {
      let out = id.info_log();
      id.delete();
      Err(out)
    }
  }
}
```

So with the Program, again we want to have some sort of thing where we just hand over two source strings and it makes it and we don't worry about all the middle steps.

```rust
pub struct ShaderProgram(pub GLuint);
impl ShaderProgram {
  pub fn from_vert_frag(vert: &str, frag: &str) -> Result<Self, String> {
    unimplemented!()
  }
}
```

But to do that we need to support all the middle steps:

```rust
/// A handle to a [Program
/// Object](https://www.khronos.org/opengl/wiki/GLSL_Object#Program_objects)
pub struct ShaderProgram(pub GLuint);
impl ShaderProgram {
  /// Allocates a new program object.
  ///
  /// Prefer [`ShaderProgram::from_vert_frag`](ShaderProgram::from_vert_frag),
  /// it makes a complete program from the vertex and fragment sources all at
  /// once.
  pub fn new() -> Option<Self> {
    let prog = unsafe { glCreateProgram() };
    if prog != 0 {
      Some(Self(prog))
    } else {
      None
    }
  }

  /// Attaches a shader object to this program object.
  pub fn attach_shader(&self, shader: &Shader) {
    unsafe { glAttachShader(self.0, shader.0) };
  }

  /// Links the various attached, compiled shader objects into a usable program.
  pub fn link_program(&self) {
    unsafe { glLinkProgram(self.0) };
  }

  /// Checks if the last linking operation was successful.
  pub fn link_success(&self) -> bool {
    let mut success = 0;
    unsafe { glGetProgramiv(self.0, GL_LINK_STATUS, &mut success) };
    success == i32::from(GL_TRUE)
  }

  /// Gets the log data for this program.
  ///
  /// This is usually used to check the message when a program failed to link.
  pub fn info_log(&self) -> String {
    let mut needed_len = 0;
    unsafe { glGetProgramiv(self.0, GL_INFO_LOG_LENGTH, &mut needed_len) };
    let mut v: Vec<u8> = Vec::with_capacity(needed_len.try_into().unwrap());
    let mut len_written = 0_i32;
    unsafe {
      glGetProgramInfoLog(
        self.0,
        v.capacity().try_into().unwrap(),
        &mut len_written,
        v.as_mut_ptr().cast(),
      );
      v.set_len(len_written.try_into().unwrap());
    }
    String::from_utf8_lossy(&v).into_owned()
  }

  /// Sets the program as the program to use when drawing.
  pub fn use_program(&self) {
    unsafe { glUseProgram(self.0) };
  }

  /// Marks the program for deletion.
  ///
  /// Note: This _does not_ immediately delete the program. If the program is
  /// currently in use it won't be deleted until it's not the active program.
  /// When a program is finally deleted and attached shaders are unattached.
  pub fn delete(self) {
    unsafe { glDeleteProgram(self.0) };
  }

  /// Takes a vertex shader source string and a fragment shader source string
  /// and either gets you a working program object or gets you an error message.
  ///
  /// This is the preferred way to create a simple shader program in the common
  /// case. It's just less error prone than doing all the steps yourself.
  pub fn from_vert_frag(vert: &str, frag: &str) -> Result<Self, String> {
    let p =
      Self::new().ok_or_else(|| "Couldn't allocate a program".to_string())?;
    let v = Shader::from_source(ShaderType::Vertex, vert)
      .map_err(|e| format!("Vertex Compile Error: {}", e))?;
    let f = Shader::from_source(ShaderType::Fragment, frag)
      .map_err(|e| format!("Fragment Compile Error: {}", e))?;
    p.attach_shader(&v);
    p.attach_shader(&f);
    p.link_program();
    v.delete();
    f.delete();
    if p.link_success() {
      Ok(p)
    } else {
      let out = format!("Program Link Error: {}", p.info_log());
      p.delete();
      Err(out)
    }
  }
}
```

Our final usage becomes:

```rust
let shader_program =
  ShaderProgram::from_vert_frag(VERT_SHADER, FRAG_SHADER).unwrap();
shader_program.use_program();
```

That's so much smaller! Very nice.

## Clearing And Drawing Arrays?

We could also wrap the clearing function, but it's small and has to go with other unsafe calls, so we'll skip it for now. We could always add it later.

We _can't_ easily make `glDrawArrays` safe, because we'd have to carefully
monitor the size of the buffer in the actively bound array buffer in the
actively bound vertex array to make sure that the call didn't make the GPU go
out of bounds. Or we could make it something like "draw these arrays", and you
pass a slice and it buffers the slice and draws it immediately. I don't really
care for either of those, so we'll just let that be unsafe too.

## Done!

* Example Code: [002-triangle-arrays2](https://github.com/rust-tutorials/learn-opengl/blob/master/examples/002-triangle-arrays2.rs)
