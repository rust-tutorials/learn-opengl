#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(unused_imports)]
#![allow(clippy::single_match)]
#![allow(clippy::zero_ptr)]

const WINDOW_TITLE: &str = "Transforms Intro";

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
use ultraviolet::*;

type Vertex = [f32; 3 + 2];
type TriIndexes = [u32; 3];

const VERTICES: [Vertex; 4] = [
  // top right
  [0.5, 0.5, 0.0, 1.0, 1.0],
  // bottom right
  [0.5, -0.5, 0.0, 1.0, 0.0],
  // bottom left
  [-0.5, -0.5, 0.0, 0.0, 0.0],
  // top left
  [-0.5, 0.5, 0.0, 0.0, 1.0],
];

const INDICES: [TriIndexes; 2] = [[0, 1, 3], [1, 2, 3]];

const VERT_SHADER: &str = r#"#version 330 core
  uniform mat4 transform;

  layout (location = 0) in vec3 pos;
  layout (location = 1) in vec2 tex;

  out vec2 frag_tex;

  void main() {
    gl_Position = transform * vec4(pos, 1.0);
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
      800,
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

  let mut logo_texture = 0;
  unsafe {
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

  let mut garris_texture = 0;
  unsafe {
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

  'main_loop: loop {
    // handle events this frame
    while let Some(event) = sdl.poll_events().and_then(Result::ok) {
      match event {
        Event::Quit(_) => break 'main_loop,
        _ => (),
      }
    }
    // now the events are clear.

    // update the "world state".
    let time = sdl.get_ticks() as f32 / 1000.0_f32;
    let transform = Mat4::from_rotation_z(time);

    // and then draw!
    unsafe {
      glClear(GL_COLOR_BUFFER_BIT);
      let transform_name = null_str!("transform").as_ptr().cast();
      let transform_loc =
        glGetUniformLocation(shader_program.0, transform_name);
      glUniformMatrix4fv(transform_loc, 1, GL_FALSE, transform.as_ptr());
      glDrawElements(GL_TRIANGLES, 6, GL_UNSIGNED_INT, null());
    }
    win.swap_window();
  }
}
