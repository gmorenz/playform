//! A vertex with and without textures attached.
use gl::types::GLfloat;
use color::Color4;
use nalgebra::na::{Vec2,Vec3};
#[cfg(test)]
use std::mem;

#[deriving(Clone, Copy, PartialEq)]
/// An untextured rendering vertex, with position and color.
pub struct ColoredVertex {
  /// The 3-d position of this vertex in world space.
  pub position: Vec3<GLfloat>,
  /// The color to apply to this vertex, in lieu of a texture.
  pub color:    Color4<GLfloat>,
}

#[test]
fn check_vertex_size() {
  assert_eq!(mem::size_of::<ColoredVertex>(), 7*4);
  assert_eq!(mem::size_of::<TextureVertex>(), 5*4);
}

impl ColoredVertex {
  /// Generates two colored triangles, representing a square, at z=0.
  /// The bounds of the square is represented by `b`.
  pub fn square(min: Vec2<GLfloat>, max: Vec2<GLfloat>, color: Color4<GLfloat>) -> [ColoredVertex, ..6] {
    let vtx = |x, y| {
        ColoredVertex { position: Vec3::new(x, y, 0.0), color: color }
      };

    [
      vtx(min.x, min.y), vtx(max.x, max.y), vtx(min.x, max.y),
      vtx(min.x, min.y), vtx(max.x, min.y), vtx(max.x, max.y),
    ]
  }
}

#[deriving(Clone, Copy, PartialEq)]
/// A point on a texture, with both a world position and a texture position.
///
/// The texture position is [0, 1].
pub struct TextureVertex {
  /// The position of this vertex in the world.
  pub world_position:  Vec3<GLfloat>,

  /// The position of this vertex on a texture. The range of valid values
  /// in each dimension is [0, 1].
  pub texture_position: Vec2<GLfloat>,
}

impl TextureVertex {
  /// Generates two textured triangles, representing a square in 2D space,
  /// at z = 0.
  /// The bounds of the square is represented by `b`.
  ///
  /// The coordinates on the texture will implicitly be the "whole thing".
  /// i.e. [(0, 0), (1, 1)].
  pub fn square(min: Vec2<GLfloat>, max: Vec2<GLfloat>) -> [TextureVertex, ..6] {
    let vtx = |x, y, tx, ty| {
        TextureVertex {
          world_position:  Vec3::new(x, y, 0.0),
          texture_position: Vec2::new(tx, ty),
        }
      };

    [
      vtx(min.x, min.y, 0.0, 0.0),
      vtx(max.x, max.y, 1.0, 1.0),
      vtx(min.x, max.y, 0.0, 1.0),

      vtx(min.x, min.y, 0.0, 0.0),
      vtx(max.x, min.y, 1.0, 0.0),
      vtx(max.x, max.y, 1.0, 1.0),
    ]
  }
}

/// A data structure which specifies how to pass data from opengl to the vertex
/// shaders.
pub struct AttribData<'a> {
  /// Cooresponds to the shader's `input variable`.
  pub name: &'a str,
  /// The size (in floats) of this attribute.
  pub size: uint,
}
