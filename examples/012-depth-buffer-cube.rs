#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(unused_imports)]
#![allow(clippy::single_match)]
#![allow(clippy::zero_ptr)]

const WINDOW_TITLE: &str = "Depth Buffer Cube";
const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 800;

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
  null_str, Buffer, BufferType, Shader, ShaderProgram, ShaderType, VertexArray,
};
use learn_opengl as learn;
use ogl33::*;
use ultraviolet::*;

type Vertex = [f32; 3 + 2];
/// Draw this with glDrawArrays(GL_TRIANGLES, 0, 36)
const CUBE_VERTICES: [Vertex; 6 * 6] = [
  // panel 1
  [-0.5, -0.5, -0.5, 0.0, 0.0],
  [0.5, -0.5, -0.5, 1.0, 0.0],
  [0.5, 0.5, -0.5, 1.0, 1.0],
  [0.5, 0.5, -0.5, 1.0, 1.0],
  [-0.5, 0.5, -0.5, 0.0, 1.0],
  [-0.5, -0.5, -0.5, 0.0, 0.0],
  // panel 2
  [-0.5, -0.5, 0.5, 0.0, 0.0],
  [0.5, -0.5, 0.5, 1.0, 0.0],
  [0.5, 0.5, 0.5, 1.0, 1.0],
  [0.5, 0.5, 0.5, 1.0, 1.0],
  [-0.5, 0.5, 0.5, 0.0, 1.0],
  [-0.5, -0.5, 0.5, 0.0, 0.0],
  // panel 3
  [-0.5, 0.5, 0.5, 1.0, 0.0],
  [-0.5, 0.5, -0.5, 1.0, 1.0],
  [-0.5, -0.5, -0.5, 0.0, 1.0],
  [-0.5, -0.5, -0.5, 0.0, 1.0],
  [-0.5, -0.5, 0.5, 0.0, 0.0],
  [-0.5, 0.5, 0.5, 1.0, 0.0],
  // panel 4
  [0.5, 0.5, 0.5, 1.0, 0.0],
  [0.5, 0.5, -0.5, 1.0, 1.0],
  [0.5, -0.5, -0.5, 0.0, 1.0],
  [0.5, -0.5, -0.5, 0.0, 1.0],
  [0.5, -0.5, 0.5, 0.0, 0.0],
  [0.5, 0.5, 0.5, 1.0, 0.0],
  // panel 5
  [-0.5, -0.5, -0.5, 0.0, 1.0],
  [0.5, -0.5, -0.5, 1.0, 1.0],
  [0.5, -0.5, 0.5, 1.0, 0.0],
  [0.5, -0.5, 0.5, 1.0, 0.0],
  [-0.5, -0.5, 0.5, 0.0, 0.0],
  [-0.5, -0.5, -0.5, 0.0, 1.0],
  // panel 6
  [-0.5, 0.5, -0.5, 0.0, 1.0],
  [0.5, 0.5, -0.5, 1.0, 1.0],
  [0.5, 0.5, 0.5, 1.0, 0.0],
  [0.5, 0.5, 0.5, 1.0, 0.0],
  [-0.5, 0.5, 0.5, 0.0, 0.0],
  [-0.5, 0.5, -0.5, 0.0, 1.0],
];

const VERT_SHADER: &str = r#"#version 330 core
  uniform mat4 model;
  uniform mat4 view;
  uniform mat4 projection;

  layout (location = 0) in vec3 pos;
  layout (location = 1) in vec2 tex;

  out vec2 frag_tex;

  void main() {
    gl_Position = projection * view * model * vec4(pos, 1.0);
    frag_tex = tex;
  }
"#;

const FRAG_SHADER: &str = r#"#version 330 core
  uniform sampler2D logo_texture;
  uniform sampler2D garris_texture;

  in vec4 frag_color;
  in vec2 frag_tex;

  out vec4 final_color;

  void main() {
    final_color = mix(texture(logo_texture, frag_tex), texture(garris_texture, frag_tex), 0.4);
  }
"#;

fn main() {
  let logo = {
    let mut f = std::fs::File::open("logo.png").unwrap();
    let mut bytes = vec![];
    std::io::Read::read_to_end(&mut f, &mut bytes).unwrap();
    let mut bitmap = imagine::png::parse_png_rgba8(&bytes).unwrap().bitmap;
    bitmap.flip_scanlines();
    bitmap
  };
  let garris = {
    let mut f = std::fs::File::open("garris_400x400.png").unwrap();
    let mut bytes = vec![];
    std::io::Read::read_to_end(&mut f, &mut bytes).unwrap();
    let mut bitmap = imagine::png::parse_png_rgba8(&bytes).unwrap().bitmap;
    bitmap.flip_scanlines();
    bitmap
  };

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

    glEnable(GL_DEPTH_TEST);
  }

  learn::clear_color(0.2, 0.3, 0.3, 1.0);

  let vao = VertexArray::new().expect("Couldn't make a VAO");
  vao.bind();

  let vbo = Buffer::new().expect("Couldn't make the vertex buffer");
  vbo.bind(BufferType::Array);
  learn::buffer_data(
    BufferType::Array,
    bytemuck::cast_slice(&CUBE_VERTICES),
    GL_STATIC_DRAW,
  );

  unsafe {
    let mut logo_texture = 0;
    glGenTextures(1, &mut logo_texture);
    glActiveTexture(GL_TEXTURE0);
    glBindTexture(GL_TEXTURE_2D, logo_texture);
    glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_S, GL_REPEAT as GLint);
    glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_T, GL_REPEAT as GLint);
    glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_LINEAR as GLint);
    glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_LINEAR as GLint);
    glTexImage2D(
      GL_TEXTURE_2D,
      0,
      GL_RGBA as GLint,
      logo.width().try_into().unwrap(),
      logo.height().try_into().unwrap(),
      0,
      GL_RGBA,
      GL_UNSIGNED_BYTE,
      logo.pixels().as_ptr().cast(),
    );
    glGenerateMipmap(GL_TEXTURE_2D);
  }

  unsafe {
    let mut garris_texture = 0;
    glGenTextures(1, &mut garris_texture);
    glActiveTexture(GL_TEXTURE1);
    glBindTexture(GL_TEXTURE_2D, garris_texture);
    glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_S, GL_REPEAT as GLint);
    glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_T, GL_REPEAT as GLint);
    glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_LINEAR as GLint);
    glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_LINEAR as GLint);
    glTexImage2D(
      GL_TEXTURE_2D,
      0,
      GL_RGBA as GLint,
      garris.width().try_into().unwrap(),
      garris.height().try_into().unwrap(),
      0,
      GL_RGBA,
      GL_UNSIGNED_BYTE,
      garris.pixels().as_ptr().cast(),
    );
    glGenerateMipmap(GL_TEXTURE_2D);
  }

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

    // tex
    glVertexAttribPointer(
      1,
      2,
      GL_FLOAT,
      GL_FALSE,
      size_of::<Vertex>().try_into().unwrap(),
      size_of::<[f32; 3]>() as *const _,
    );
    glEnableVertexAttribArray(1);

    let logo_name = null_str!("logo_texture").as_ptr().cast();
    glUniform1i(glGetUniformLocation(shader_program.0, logo_name), 0);

    let garris_name = null_str!("garris_texture").as_ptr().cast();
    glUniform1i(glGetUniformLocation(shader_program.0, garris_name), 1);
  }

  let model_loc = unsafe {
    let name = null_str!("model").as_ptr().cast();
    glGetUniformLocation(shader_program.0, name)
  };
  let view_loc = unsafe {
    let name = null_str!("view").as_ptr().cast();
    glGetUniformLocation(shader_program.0, name)
  };
  let projection_loc = unsafe {
    let name = null_str!("projection").as_ptr().cast();
    glGetUniformLocation(shader_program.0, name)
  };

  let view = Mat4::from_translation(Vec3::new(0.0, 0.0, -2.0));
  unsafe { glUniformMatrix4fv(view_loc, 1, GL_FALSE, view.as_ptr()) };

  let projection = ultraviolet::projection::perspective_gl(
    45.0_f32.to_radians(),
    (WINDOW_WIDTH as f32) / (WINDOW_HEIGHT as f32),
    0.1,
    100.0,
  );
  unsafe {
    glUniformMatrix4fv(projection_loc, 1, GL_FALSE, projection.as_ptr())
  };

  'main_loop: loop {
    // handle events this frame
    while let Some((event, _timestamp)) = sdl.poll_events() {
      match event {
        Event::Quit => break 'main_loop,
        _ => (),
      }
    }
    // now the events are clear.

    // update the "world state".
    let time = sdl.get_ticks() as f32 / 1000.0_f32;
    let model = Mat4::from_rotation_y(1.0)
      * Mat4::from_rotation_x(0.5)
      * Mat4::from_rotation_z(time);

    // and then draw!
    unsafe {
      glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);

      glUniformMatrix4fv(model_loc, 1, GL_FALSE, model.as_ptr());

      glDrawArrays(GL_TRIANGLES, 0, 36);
    }
    win.swap_window();
  }
}
