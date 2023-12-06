#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(unused_imports)]
#![allow(clippy::single_match)]
#![allow(clippy::zero_ptr)]

const WINDOW_TITLE: &str = "More Attributes";

use beryllium::{
  events::Event,
  init::InitFlags,
  video::{CreateWinArgs, GlContextFlags, GlProfile, GlSwapInterval},
  *,
};
use core::{
  convert::{TryFrom, TryInto},
  mem::{size_of, size_of_val},
  ptr::null,
};
use learn::{
  Buffer, BufferType, Shader, ShaderProgram, ShaderType, VertexArray,
};
use learn_opengl as learn;
use ogl33::*;

type Vertex = [f32; 6];
type TriIndexes = [u32; 3];

const VERTICES: [Vertex; 4] = [
  [0.5, 0.5, 0.0, 1.0, 0.0, 0.0],
  [0.5, -0.5, 0.0, 0.0, 1.0, 0.0],
  [-0.5, -0.5, 0.0, 0.0, 0.0, 1.0],
  [-0.5, 0.5, 0.0, 0.2, 0.3, 0.4],
];

const INDICES: [TriIndexes; 2] = [[0, 1, 3], [1, 2, 3]];

const VERT_SHADER: &str = r#"#version 330 core
  layout (location = 0) in vec3 pos;
  layout (location = 1) in vec3 color;

  out vec4 frag_color;

  void main() {
    gl_Position = vec4(pos.x, pos.y, pos.z, 1.0);
    frag_color = vec4(color, 1.0);
  }
"#;

const FRAG_SHADER: &str = r#"#version 330 core
  in vec4 frag_color;

  out vec4 final_color;

  void main() {
    final_color = frag_color;
  }
"#;

fn main() {
  let sdl = Sdl::init(InitFlags::EVERYTHING);
  sdl.set_gl_context_major_version(3).unwrap();
  sdl.set_gl_context_minor_version(3).unwrap();
  sdl.set_gl_profile(GlProfile::Core).unwrap();
  let mut flags = GlContextFlags::default();
  if cfg!(target_os = "macos") {
    flags |= GlContextFlags::FORWARD_COMPATIBLE;
  }
  if cfg!(debug_asserts) {
    flags |= GlContextFlags::DEBUG;
  }
  sdl.set_gl_context_flags(flags).unwrap();

  let win = sdl
    .create_gl_window(CreateWinArgs {
      title: WINDOW_TITLE,
      width: 800,
      height: 600,
      ..Default::default()
    })
    .expect("couldn't make a window and context");
  win.set_swap_interval(GlSwapInterval::Vsync).unwrap();

  unsafe {
    load_gl_with(|f_name| win.get_proc_address(f_name.cast()));
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
    // position
    glVertexAttribPointer(
      0,
      3,
      GL_FLOAT,
      GL_FALSE,
      size_of::<Vertex>().try_into().unwrap(),
      0 as *const _,
    );
    glEnableVertexAttribArray(0);

    // color
    glVertexAttribPointer(
      1,
      3,
      GL_FLOAT,
      GL_FALSE,
      size_of::<Vertex>().try_into().unwrap(),
      size_of::<[f32; 3]>() as *const _,
    );
    glEnableVertexAttribArray(1);
  }

  'main_loop: loop {
    // handle events this frame
    while let Some((event, _timestamp)) = sdl.poll_events() {
      match event {
        Event::Quit => break 'main_loop,
        _ => (),
      }
    }
    // now the events are clear.

    // here's where we could change the world state if we had some.

    // and then draw!
    unsafe {
      glClear(GL_COLOR_BUFFER_BIT);
      glDrawElements(GL_TRIANGLES, 6, GL_UNSIGNED_INT, null());
    }
    win.swap_window();
  }
}
