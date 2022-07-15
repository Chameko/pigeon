use super::{Breakdown, Drawable, Texture};
use crate::pipeline::quad::{QuadPipe, QuadVertex};
use euclid::{Point3D, Rotation3D, Size2D, Translation3D};
use parrot::transform::{ObjectSpace, WorldSpace};
use std::rc::Rc;

/// Basic textured rectangle.

/// Basic textured rectangle. Uses the same position and size system as [`super::primative::Rectangle`]
/// with an origin at its centre and a width and height
/// Uses the [`QuadPipe`] pipeline
pub struct Sprite {
    /// The centre of the sprite
    pub origin: Point3D<f32, WorldSpace>,
    /// The size of the sprite
    pub size: Size2D<f32, ObjectSpace>,
    /// The rotation of the sprite
    pub rotation: Rotation3D<f32, ObjectSpace, ObjectSpace>,
    /// The texture of the sprite
    pub texture: Rc<Texture>,
}

impl Sprite {
    /// Create a new sprite
    pub fn new(
        origin: impl Into<Point3D<f32, WorldSpace>>,
        size: impl Into<Size2D<f32, ObjectSpace>>,
        texture: Rc<Texture>,
    ) -> Self {
        Self {
            origin: origin.into(),
            size: size.into(),
            rotation: Rotation3D::identity(),
            texture,
        }
    }

    // Rotate the sprite
    pub fn rotate(&mut self, rotation: Rotation3D<f32, ObjectSpace, ObjectSpace>) {
        self.rotation = rotation;
    }

    /// Translate the sprite
    pub fn translate(&mut self, translation: Translation3D<f32, WorldSpace, WorldSpace>) {
        self.origin = translation.transform_point3d(&self.origin);
    }

    /// Set the sprites size
    pub fn scale(&mut self, size: Size2D<f32, ObjectSpace>) {
        self.size = size;
    }

    /// Update the texture of the sprite
    pub fn update_texture(&mut self, texture: Rc<Texture>) {
        self.texture = texture;
    }
}

impl Drawable for Sprite {
    type Pipeline = QuadPipe;

    fn breakdown(&self) -> Breakdown<QuadVertex> {
        let mut tl: Point3D<f32, ObjectSpace> = Point3D::new(
            -self.size.width / 2.0,
            self.size.height / 2.0,
            self.origin.z,
        );
        let mut tr: Point3D<f32, ObjectSpace> =
            Point3D::new(self.size.width / 2.0, self.size.height / 2.0, self.origin.z);
        let mut bl: Point3D<f32, ObjectSpace> = Point3D::new(
            -self.size.width / 2.0,
            -self.size.height / 2.0,
            self.origin.z,
        );
        let mut br: Point3D<f32, ObjectSpace> = Point3D::new(
            self.size.width / 2.0,
            -self.size.height / 2.0,
            self.origin.z,
        );
        // Rotate each of the points (this must be done in object space)
        for vert in [&mut tl, &mut tr, &mut bl, &mut br] {
            *vert = self.rotation.transform_point3d(*vert);
            vert.x = vert.x + self.origin.x;
            vert.y = vert.y + self.origin.y;
        }
        let vertices = vec![
            QuadVertex::new_from_tuple(tl.to_tuple(), (0.0, 0.0)),
            QuadVertex::new_from_tuple(tr.to_tuple(), (1.0, 0.0)),
            QuadVertex::new_from_tuple(bl.to_tuple(), (0.0, 1.0)),
            QuadVertex::new_from_tuple(br.to_tuple(), (1.0, 1.0)),
        ];

        Breakdown {
            vertices,
            indicies: vec![0, 1, 3, 0, 3, 2],
            texture: Some(self.texture.clone()),
        }
    }
}
