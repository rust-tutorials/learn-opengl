#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(unused_imports)]
#![allow(clippy::single_match)]
#![allow(clippy::zero_ptr)]

const WINDOW_TITLE: &str = "Movement";
const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 800;

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
use std::collections::HashSet;
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

const CUBE_POSITIONS: [Vec3; 10] = [
  Vec3 { x: 0.0, y: 0.0, z: 0.0 },
  Vec3 { x: 2.0, y: 5.0, z: -15.0 },
  Vec3 { x: -1.5, y: -2.2, z: -2.5 },
  Vec3 { x: -3.8, y: -2.0, z: -12.3 },
  Vec3 { x: 2.4, y: -0.4, z: -3.5 },
  Vec3 { x: -1.7, y: 3.0, z: -7.5 },
  Vec3 { x: 1.3, y: -2.0, z: -2.5 },
  Vec3 { x: 1.5, y: 2.0, z: -2.5 },
  Vec3 { x: 1.5, y: 0.2, z: -1.5 },
  Vec3 { x: -1.3, y: 1.0, z: -1.5 },
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
      WINDOW_WIDTH,
      WINDOW_HEIGHT,
      WindowFlags::Shown,
    )
    .expect("couldn't make a window and context");
  win.set_swap_interval(SwapInterval::Vsync);

  unsafe {
    load_gl_with(|f_name| win.get_proc_address(f_name));

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

  let projection = ultraviolet::projection::perspective_gl(
    45.0_f32.to_radians(),
    (WINDOW_WIDTH as f32) / (WINDOW_HEIGHT as f32),
    0.1,
    100.0,
  );
  unsafe {
    glUniformMatrix4fv(projection_loc, 1, GL_FALSE, projection.as_ptr())
  };

  let mut camera =
    EulerFPSCamera::at_position(Vec3 { x: 0.0, y: 0.0, z: -3.0 });
  let camera_speed = 100.0;
  sdl.set_relative_mouse_mode(true).unwrap();
  let mut keys_held = HashSet::new();
  let mut last_time = 0.0;

  'main_loop: loop {
    // handle events this frame
    while let Some(event) = sdl.poll_events().and_then(Result::ok) {
      match event {
        Event::Quit(_) => break 'main_loop,
        Event::MouseMotion(MouseMotionEvent { x_delta, y_delta, .. }) => {
          let d_yaw_deg = -x_delta as f32 * 0.1;
          let d_pitch_deg = -y_delta as f32 * 0.1;
          camera.update_orientation(d_pitch_deg, d_yaw_deg);
        }
        Event::Keyboard(KeyboardEvent {
          is_pressed,
          key: KeyInfo { keycode, .. },
          ..
        }) => {
          if is_pressed {
            keys_held.insert(keycode);
          } else {
            keys_held.remove(&keycode);
          }
        }
        _ => (),
      }
    }
    // now the events are clear.

    // update the "world state".
    let time = sdl.get_ticks() as f32 / 10_000.0_f32;
    let delta_time = time - last_time;
    last_time = time;

    camera.update_position(&keys_held, camera_speed * delta_time);

    let view: Mat4 = camera.make_view_matrix();

    // and then draw!
    unsafe {
      glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);

      glUniformMatrix4fv(view_loc, 1, GL_FALSE, view.as_ptr());

      for (i, position) in CUBE_POSITIONS.iter().copied().enumerate() {
        let model = Mat4::from_translation(position)
          * Mat4::from_rotation_y(3.0)
          * Mat4::from_rotation_x((1.0 + i as f32) * 0.8)
          * Mat4::from_rotation_z(time * (1.0 + i as f32));

        glUniformMatrix4fv(model_loc, 1, GL_FALSE, model.as_ptr());

        glDrawArrays(GL_TRIANGLES, 0, 36);
      }
    }
    win.swap_window();
  }
}

/// Acts like a normal "FPS" camera, capped at +/- 89 degrees, no roll.
#[derive(Debug, Clone, Copy)]
pub struct EulerFPSCamera {
  /// Camera position, free free to directly update at any time.
  pub position: Vec3,
  pitch_deg: f32,
  yaw_deg: f32,
}
impl EulerFPSCamera {
  const UP: Vec3 = Vec3 { x: 0.0, y: 1.0, z: 0.0 };

  fn make_front(&self) -> Vec3 {
    let pitch_rad = f32::to_radians(self.pitch_deg);
    let yaw_rad = f32::to_radians(self.yaw_deg);
    Vec3 {
      x: yaw_rad.sin() * pitch_rad.cos(),
      y: pitch_rad.sin(),
      z: yaw_rad.cos() * pitch_rad.cos(),
    }
  }

  /// Adjusts the camera's orientation.
  ///
  /// Input deltas should be in _degrees_, pitch is capped at +/- 89 degrees.
  pub fn update_orientation(&mut self, d_pitch_deg: f32, d_yaw_deg: f32) {
    self.pitch_deg = (self.pitch_deg + d_pitch_deg).max(-89.0).min(89.0);
    self.yaw_deg = (self.yaw_deg + d_yaw_deg) % 360.0;
  }

  /// Updates the position using WASDQE controls.
  ///
  /// The "forward" vector is relative to the current orientation.
  pub fn update_position(&mut self, keys: &HashSet<Keycode>, distance: f32) {
    let forward = self.make_front();
    let cross_normalized = forward.cross(Self::UP).normalized();
    let mut move_vector =
      keys.iter().copied().fold(Vec3 { x: 0.0, y: 0.0, z: 0.0 }, |vec, key| {
        match key {
          Keycode::W => vec + forward,
          Keycode::S => vec - forward,
          Keycode::A => vec - cross_normalized,
          Keycode::D => vec + cross_normalized,
          Keycode::E => vec + Self::UP,
          Keycode::Q => vec - Self::UP,
          _ => vec,
        }
      });
    if !(move_vector.x == 0.0 && move_vector.y == 0.0 && move_vector.z == 0.0) {
      move_vector = move_vector.normalized();
      self.position += move_vector * distance;
    }
  }

  /// Generates the current view matrix for this camera.
  pub fn make_view_matrix(&self) -> Mat4 {
    Mat4::look_at(self.position, self.position + self.make_front(), Self::UP)
  }

  /// Makes a new camera at the position specified and Pitch/Yaw of `0.0`.
  pub const fn at_position(position: Vec3) -> Self {
    Self { position, pitch_deg: 0.0, yaw_deg: 0.0 }
  }
}
