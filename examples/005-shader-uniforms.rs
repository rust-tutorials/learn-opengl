#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(unused_imports)]
#![allow(clippy::single_match)]
#![allow(clippy::zero_ptr)]

const WINDOW_TITLE: &str = "Shader Uniforms";

use beryllium::*;
use core::{
  convert::{TryFrom, TryInto},
  mem::{size_of, size_of_val},
  ptr::null,
};
use learn::{
  null_str, Buffer, BufferType, Shader, ShaderProgram, ShaderType, VertexArray,
};
use learn_opengl as learn;
use ogl33::*;

type Vertex = [f32; 3];
type TriIndexes = [u32; 3];

const VERTICES: [Vertex; 4] =
  [[0.5, 0.5, 0.0], [0.5, -0.5, 0.0], [-0.5, -0.5, 0.0], [-0.5, 0.5, 0.0]];

const INDICES: [TriIndexes; 2] = [[0, 1, 3], [1, 2, 3]];

const VERT_SHADER: &str = r#"#version 330 core
  layout (location = 0) in vec3 pos;

  void main() {
    gl_Position = vec4(pos.x, pos.y, pos.z, 1.0);
  }
"#;

const FRAG_SHADER: &str = r#"#version 330 core
  uniform vec4 uni_color;

  out vec4 final_color;

  void main() {
    final_color = uni_color;
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
    load_gl_with(|f_name| win.get_proc_address(f_name));
  }

  learn::clear_color(0.2, 0.3, 0.3, 1.0);

  let vao = VertexArray::new().expect("Couldn't make a VAO");
  vao.bind();

  let vbo = Buffer::new().expect("Couldn't make the vertex buffer");
  vbo.bind(BufferType::Array);
  learn::buffer_data(
    BufferType::Array,
    bytemuck::cast_slice(&VERTICES),
    GL_STATIC_DRAW,
  );

  let ebo = Buffer::new().expect("Couldn't make the element buffer.");
  ebo.bind(BufferType::ElementArray);
  learn::buffer_data(
    BufferType::ElementArray,
    bytemuck::cast_slice(&INDICES),
    GL_STATIC_DRAW,
  );

  let shader_program =
    ShaderProgram::from_vert_frag(VERT_SHADER, FRAG_SHADER).unwrap();
  shader_program.use_program();

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

  let p: *const c_char = null_str!("uni_color").as_ptr().cast();
  let uni_color_loc = unsafe { glGetUniformLocation(shader_program.0, p) };

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
    let time = sdl.get_ticks() as f32 / 1000.0_f32;
    let green = (f32::sin(time) / 2.0) + 0.5;

    // and then draw!
    unsafe {
      glClear(GL_COLOR_BUFFER_BIT);
      glUniform4f(uni_color_loc, 0.1, green, 0.1, 1.0);
      glDrawElements(GL_TRIANGLES, 6, GL_UNSIGNED_INT, null());
    }
    win.swap_window();
  }
}
