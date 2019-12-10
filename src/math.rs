#![allow(missing_docs)]

//! Module for Math that's needed during the Learn OpenGL Rust tutorials.
//!
//! This is _not_ a complete math library. It's enough to get the job done and
//! that's about it. In fact, you could say that it's in a sense deliberately
//! limited. I haven't put anything in here that isn't covered in the book, that
//! way I don't accidentally use a function in an example that I haven't covered
//! in the book.
//!
//! If you want a more complete math library feel free to try one of these
//! crates:
//!
//! * [nalgebra-glm](https://docs.rs/nalgebra-glm) is a handy wrapper crate that
//!   lets you potentially ease into using the full
//!   [nalgebra](https://nalgebra.org/) crate. `nalgebra-glm` is styled after
//!   the GLM math library for C++, so the naming will match up with a lot of
//!   the math calls you'll see if you learn about OpenGL in C++ blogs and
//!   books. `nalgebra` itself is the crate you need to use if you want to some
//!   day use the
//!   [nphysics2d](https://www.nphysics.org/rustdoc/nphysics2d/index.html) or
//!   [nphysics3d](https://www.nphysics.org/rustdoc/nphysics3d/index.html)
//!   crates. This family of crates is very powerful, but the code is extremely
//!   generic which means that both the compile times and the recompile times
//!   are usually monstrous. I only suggest this crate family if you
//!   specifically want to use `nphysics2d` or `nphysics3d`. Otherwise you
//!   should probably look elsewhere.
//! * [glam](https://docs.rs/glam) is a math lib that is non-generic, and
//!   focuses on using explicit SIMD to make things as fast as possible. We'll
//!   call this the "fast, normal" option. If you don't know what you should be
//!   using, you probably want to use this lib as your default lib.
//! * [ultraviolet](https://docs.rs/ultraviolet) is where I'm saving the best
//!   for last. It has types for singular math objects but also for batches of
//!   math objects. For example, in addition to `Vec3` being full of `f32`
//!   values, there's also a `Wec3` which is full of `f32x4` values. As a whole
//!   the `Wec3` is four `Vec3` values in one, but it can generally perform ops
//!   in the same amount of time. This means that _if you can architect your
//!   code for it_, you can get a significant speed boost. Not actually the full
//!   4x speed boost, but about 1.5x to 2x (compared to `glam`, which is
//!   generally the next fastest lib). However, making sure that your code is
//!   working in blocks of 4 things at a time is a little more tricky, so I
//!   consider this to be an "advanced" sort of choice.

#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Vec3 {
  pub x: f32,
  pub y: f32,
  pub z: f32,
}

pub const fn vec3(x: f32, y: f32, z: f32) -> Vec3 {
  Vec3 { x, y, z }
}

impl Vec3 {
  pub fn cross(self, rhs: Self) -> Self {
    unimplemented!()
  }

  pub fn normalized(self) -> Self {
    unimplemented!()
  }
}

impl core::ops::Add for Vec3 {
  type Output = Self;

  fn add(self, rhs: Self) -> Self {
    unimplemented!()
  }
}
impl core::ops::AddAssign for Vec3 {
  fn add_assign(&mut self, rhs: Self) {
    *self = *self + rhs
  }
}

impl core::ops::Sub for Vec3 {
  type Output = Self;

  fn sub(self, rhs: Self) -> Self {
    unimplemented!()
  }
}
impl core::ops::SubAssign for Vec3 {
  fn sub_assign(&mut self, rhs: Self) {
    *self = *self - rhs
  }
}

impl core::ops::Mul<f32> for Vec3 {
  type Output = Self;

  fn mul(self, rhs: f32) -> Self {
    unimplemented!()
  }
}
impl core::ops::MulAssign<f32> for Vec3 {
  fn mul_assign(&mut self, rhs: f32) {
    *self = *self * rhs
  }
}
impl core::ops::Mul<Vec3> for f32 {
  type Output = Vec3;

  fn mul(self, rhs: Vec3) -> Vec3 {
    rhs * self
  }
}

#[repr(C, align(16))]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Vec4 {
  pub x: f32,
  pub y: f32,
  pub z: f32,
  pub w: f32,
}

#[repr(C, align(16))]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Mat4 {
  pub x: Vec4,
  pub y: Vec4,
  pub z: Vec4,
  pub w: Vec4,
}

impl Mat4 {
  pub fn identity() -> Self {
    unimplemented!()
  }
  pub fn translate(delta: Vec3) -> Self {
    unimplemented!()
  }
  pub fn rotate_x(radians: f32) -> Self {
    unimplemented!()
  }
  pub fn rotate_y(radians: f32) -> Self {
    unimplemented!()
  }
  pub fn rotate_z(radians: f32) -> Self {
    unimplemented!()
  }
  pub fn euler_angles(yaw: f32, pitch: f32, roll: f32) -> Self {
    unimplemented!()
  }
  pub fn look_at(position: Vec3, eye: Vec3, up: Vec3) -> Self {
    unimplemented!()
  }
  pub fn as_ptr(&self) -> *const f32 {
    unimplemented!()
  }
}

pub fn perspective_view(
  view_radians: f32,
  aspect_ratio: f32,
  near: f32,
  far: f32,
) -> Mat4 {
  unimplemented!()
}

pub fn orthographic_view(
  left: f32,
  right: f32,
  bottom: f32,
  top: f32,
  near: f32,
  far: f32,
) -> Mat4 {
  unimplemented!()
}

impl core::ops::Mul for Mat4 {
  type Output = Self;

  fn mul(self, rhs: Self) -> Self {
    unimplemented!()
  }
}
