#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(clippy::single_match)]
#![allow(unused_imports)]
#![allow(clippy::zero_ptr)]

const WINDOW_TITLE: &str = "Triangle: Draw Arrays Cleaned Up";

use beryllium::*;
use core::{
  convert::{TryFrom, TryInto},
  mem::{size_of, size_of_val},
};
use learn::{
  BufferType, Shader, ShaderProgram, ShaderType, VertexArray, VertexBuffer,
};
use learn_opengl as learn;
use ogl33 as gl;

type Vertex = [f32; 3];

const VERTICES: [Vertex; 3] =
  [[-0.5, -0.5, 0.0], [0.5, -0.5, 0.0], [0.0, 0.5, 0.0]];

const VERT_SHADER: &str = r#"#version 330 core

layout (location = 0) in vec3 pos;

void main() {
  gl_Position = vec4(pos.x, pos.y, pos.z, 1.0);
}
"#;

const FRAG_SHADER: &str = r#"#version 330 core
out vec4 final_color;

void main() {
  final_color = vec4(1.0, 0.5, 0.2, 1.0);
}
"#;

fn main() {
  let sdl = SDL::init(InitFlags::Everything).expect("couldn't start SDL");
  sdl.gl_set_attribute(SdlGlAttr::MajorVersion, 3).unwrap();
  sdl.gl_set_attribute(SdlGlAttr::MinorVersion, 3).unwrap();
  sdl.gl_set_attribute(SdlGlAttr::Profile, GlProfile::Core).unwrap();
  #[cfg(target_os = "macos")]
  {
    sdl
      .gl_set_attribute(SdlGlAttr::Flags, ContextFlag::ForwardCompatible)
      .unwrap();
  }

  let win = sdl
    .create_gl_window(
      WINDOW_TITLE,
      WindowPosition::Centered,
      800,
      600,
      WindowFlags::Shown,
    )
    .expect("couldn't make a window and context");
  win.set_swap_interval(SwapInterval::Vsync);

  unsafe {
    gl::load_with(|f_name| win.get_proc_address(f_name));
  }

  learn::clear_color(0.2, 0.3, 0.3, 1.0);

  let vao = VertexArray::new().expect("Couldn't make a VAO");
  vao.bind();

  let vbo = VertexBuffer::new().expect("Couldn't make a VBO");
  vbo.bind(BufferType::Array);
  learn::buffer_data(
    BufferType::Array,
    bytemuck::cast_slice(&VERTICES),
    gl::STATIC_DRAW,
  );

  let vertex_shader =
    Shader::new(ShaderType::Vertex).expect("failed to make a vertex shader");
  vertex_shader.set_source(VERT_SHADER);
  vertex_shader.compile();
  if !vertex_shader.compile_success() {
    panic!("Vertex Compile Error: {}", vertex_shader.info_log());
  }

  let fragment_shader =
    Shader::new(ShaderType::Fragment).expect("failed to make a vertex shader");
  fragment_shader.set_source(FRAG_SHADER);
  fragment_shader.compile();
  if !fragment_shader.compile_success() {
    panic!("Fragment Compile Error: {}", fragment_shader.info_log());
  }

  let shader_program =
    ShaderProgram::new().expect("couldn't make a shader program");
  shader_program.attach_shader(&vertex_shader);
  shader_program.attach_shader(&fragment_shader);
  shader_program.link_program();
  if !shader_program.link_success() {
    panic!("Shader Link Error: {}", shader_program.info_log());
  }
  vertex_shader.delete();
  fragment_shader.delete();
  shader_program.use_program();

  unsafe {
    gl::VertexAttribPointer(
      0,
      3,
      gl::FLOAT,
      gl::FALSE,
      size_of::<Vertex>().try_into().unwrap(),
      0 as *const _,
    );
    gl::EnableVertexAttribArray(0);
  }

  'main_loop: loop {
    // handle events this frame
    while let Some(event) = sdl.poll_events().and_then(Result::ok) {
      match event {
        Event::Quit(_) => break 'main_loop,
        _ => (),
      }
    }
    // now the events are clear.

    // here's where we could change the world state if we had some.

    // and then draw!
    unsafe {
      gl::Clear(gl::COLOR_BUFFER_BIT);
      gl::DrawArrays(gl::TRIANGLES, 0, 3);
      win.swap_window();
    }
  }
}
