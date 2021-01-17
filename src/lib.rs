#![allow(unused_imports)]
#![warn(missing_docs)]

//! Simplistic OpenGL wrappers, for use with the Learn-OpenGL book.
//!
//! Please do **not** think that this is a perfectly solid and complete wrapper
//! for OpenGL!
//!
//! It is mostly focused on the parts of OpenGL that can be _easily_ wrapped to
//! give the programmer a good leg up while doing so. Any parts that would be
//! hard or complicated to make safe have just been skipped over. That would
//! take a lot of time away from covering OpenGL itself. Usually the exact
//! design comes down to personal preference. I'd rather spend time on adding to
//! the book, and you can just use some `unsafe` blocks here and there.

/*

NEEDS FIXING:

We need to fix these once Fusha fixes the matrix stuff (lesson 10 and onward)

texture id variables should go inside the block because we don't use them later.
(check all lessons from 7 onward)

TODO:

016 mouse-wheel Zoom on the camera
017 Free Camera? (allowing roll)
--- end of arc 1

*/

use core::convert::{TryFrom, TryInto};
use ogl33::*;

/// Takes a string literal and concatenates a null byte onto the end.
#[macro_export]
macro_rules! null_str {
  ($lit:literal) => {{
    // "type check" the input
    const _: &str = $lit;
    concat!($lit, "\0")
  }};
}

/// Sets the color to clear to when clearing the screen.
pub fn clear_color(r: f32, g: f32, b: f32, a: f32) {
  unsafe { glClearColor(r, g, b, a) }
}

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

/// The types of buffer object that you can have.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BufferType {
  /// Array Buffers holds arrays of vertex data for drawing.
  Array = GL_ARRAY_BUFFER as isize,
  /// Element Array Buffers hold indexes of what vertexes to use for drawing.
  ElementArray = GL_ELEMENT_ARRAY_BUFFER as isize,
}

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

/// The types of shader object.
pub enum ShaderType {
  /// Vertex shaders determine the position of geometry within the screen.
  Vertex = GL_VERTEX_SHADER as isize,
  /// Fragment shaders determine the color output of geometry.
  ///
  /// Also other values, but mostly color.
  Fragment = GL_FRAGMENT_SHADER as isize,
}

/// A handle to a [Shader
/// Object](https://www.khronos.org/opengl/wiki/GLSL_Object#Shader_objects)
pub struct Shader(pub GLuint);
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

/// The polygon display modes you can set.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PolygonMode {
  /// Just show the points.
  Point = GL_POINT as isize,
  /// Just show the lines.
  Line = GL_LINE as isize,
  /// Fill in the polygons.
  Fill = GL_FILL as isize,
}

/// Sets the font and back polygon mode to the mode given.
pub fn polygon_mode(mode: PolygonMode) {
  unsafe { glPolygonMode(GL_FRONT_AND_BACK, mode as GLenum) };
}
