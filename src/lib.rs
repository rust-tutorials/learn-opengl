#![allow(unused_imports)]

use core::convert::{TryFrom, TryInto};
use gl::{GLenum, GLuint};
use ogl33 as gl;

pub fn clear_color(r: f32, g: f32, b: f32, a: f32) {
  unsafe { gl::ClearColor(r, g, b, a) }
}

pub struct VertexArray(pub GLuint);
impl VertexArray {
  /// Creates a new vertex array object
  pub fn new() -> Option<Self> {
    let mut vao = 0;
    unsafe { gl::GenVertexArrays(1, &mut vao) };
    if vao != 0 {
      Some(Self(vao))
    } else {
      None
    }
  }

  /// Bind this vertex array as the current vertex array object
  pub fn bind(&self) {
    unsafe { gl::BindVertexArray(self.0) }
  }

  /// Clear the current vertex array object binding.
  pub fn clear_binding() {
    unsafe { gl::BindVertexArray(0) }
  }
}

#[derive(Debug, Clone, Copy)]
pub enum BufferType {
  Array = gl::ARRAY_BUFFER as isize,
}

pub struct VertexBuffer(pub GLuint);
impl VertexBuffer {
  /// Makes a new vertex buffer
  pub fn new() -> Option<Self> {
    let mut vbo = 0;
    unsafe {
      gl::GenBuffers(1, &mut vbo);
    }
    if vbo != 0 {
      Some(Self(vbo))
    } else {
      None
    }
  }

  /// Bind this vertex buffer for the given type
  pub fn bind(&self, ty: BufferType) {
    unsafe { gl::BindBuffer(ty as GLenum, self.0) }
  }

  /// Clear the current vertex buffer binding for the given type.
  pub fn clear_binding(ty: BufferType) {
    unsafe { gl::BindBuffer(ty as GLenum, 0) }
  }
}

pub fn buffer_data(ty: BufferType, data: &[u8], usage: GLenum) {
  unsafe {
    gl::BufferData(
      ty as GLenum,
      data.len().try_into().unwrap(),
      data.as_ptr().cast(),
      usage,
    );
  }
}

pub enum ShaderType {
  Vertex = gl::VERTEX_SHADER as isize,
  Fragment = gl::FRAGMENT_SHADER as isize,
}

pub struct Shader(pub GLuint);
impl Shader {
  pub fn new(ty: ShaderType) -> Option<Self> {
    let shader = unsafe { gl::CreateShader(ty as GLenum) };
    if shader != 0 {
      Some(Self(shader))
    } else {
      None
    }
  }

  pub fn set_source(&self, src: &str) {
    unsafe {
      gl::ShaderSource(
        self.0,
        1,
        &(src.as_bytes().as_ptr().cast()),
        &(src.len().try_into().unwrap()),
      );
    }
  }

  pub fn compile(&self) {
    unsafe { gl::CompileShader(self.0) };
  }

  pub fn compile_success(&self) -> bool {
    let mut compiled = 0;
    unsafe { gl::GetShaderiv(self.0, gl::COMPILE_STATUS, &mut compiled) };
    compiled == i32::from(gl::TRUE)
  }

  pub fn info_log(&self) -> String {
    let mut needed_len = 0;
    unsafe { gl::GetShaderiv(self.0, gl::INFO_LOG_LENGTH, &mut needed_len) };
    let mut v: Vec<u8> = Vec::with_capacity(needed_len.try_into().unwrap());
    let mut len_written = 0_i32;
    unsafe {
      gl::GetShaderInfoLog(
        self.0,
        v.capacity().try_into().unwrap(),
        &mut len_written,
        v.as_mut_ptr().cast(),
      );
      v.set_len(len_written.try_into().unwrap());
    }
    String::from_utf8_lossy(&v).into_owned()
  }

  pub fn delete(self) {
    unsafe { gl::DeleteShader(self.0) };
  }
}

pub struct ShaderProgram(pub GLuint);
impl ShaderProgram {
  pub fn new() -> Option<Self> {
    let prog = unsafe { gl::CreateProgram() };
    if prog != 0 {
      Some(Self(prog))
    } else {
      None
    }
  }

  pub fn attach_shader(&self, shader: &Shader) {
    unsafe { gl::AttachShader(self.0, shader.0) };
  }

  pub fn link_program(&self) {
    unsafe { gl::LinkProgram(self.0) };
  }

  pub fn link_success(&self) -> bool {
    let mut success = 0;
    unsafe { gl::GetProgramiv(self.0, gl::LINK_STATUS, &mut success) };
    success == i32::from(gl::TRUE)
  }

  pub fn info_log(&self) -> String {
    let mut needed_len = 0;
    unsafe { gl::GetProgramiv(self.0, gl::INFO_LOG_LENGTH, &mut needed_len) };
    let mut v: Vec<u8> = Vec::with_capacity(needed_len.try_into().unwrap());
    let mut len_written = 0_i32;
    unsafe {
      gl::GetProgramInfoLog(
        self.0,
        v.capacity().try_into().unwrap(),
        &mut len_written,
        v.as_mut_ptr().cast(),
      );
      v.set_len(len_written.try_into().unwrap());
    }
    String::from_utf8_lossy(&v).into_owned()
  }

  pub fn use_program(&self) {
    unsafe { gl::UseProgram(self.0) };
  }
}
