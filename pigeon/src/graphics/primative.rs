use parrot::{
    transform::{ObjectSpace, WorldSpace}, Rgba,
};
use euclid::{
    Point3D, Size2D, Rect, Rotation3D, Translation3D, Transform3D,
};
use super::{Drawable, Breakdown};
use crate::pipeline::{triangle::TriangleVertex, TrianglePipe};

/// Various primatives that can be drawn using pigeons built in pipelines

/// A Basic rectangle, represented by an origin (the centre of the rectangle) and a size relative to the origin.
/// Uses the [`TrianglePipe`] pipeline
#[derive(Debug, Clone)]
pub struct Rectangle {
    /// The centre of hte rectangle
    pub origin: Point3D<f32, WorldSpace>,
    /// The size of the rectangle
    pub size: Size2D<f32, ObjectSpace>,
    /// The roation of the rectangle
    pub rotation: Rotation3D<f32, ObjectSpace, ObjectSpace>,
    /// The color of the rectangle
    pub color: Rgba,
}

impl Rectangle {
    /// Create a new rectangle
    pub fn new(origin: impl Into<Point3D<f32, WorldSpace>>, size: impl Into<Size2D<f32, ObjectSpace>>, color: impl Into<Rgba>) -> Self{
        Self {
            origin: origin.into(),
            size: size.into(),
            rotation: Rotation3D::identity(),
            color: color.into()
        }
    }

    /// Rotate the rectangle
    pub fn rotate(&mut self, rotation: Rotation3D<f32, ObjectSpace, ObjectSpace>) {
        self.rotation = rotation ;
    }

    /// Translate the rectangle
    pub fn translate(&mut self, translation: Translation3D<f32, WorldSpace, WorldSpace>) {
        self.origin = translation.transform_point3d(&self.origin);
    }

    /// Set the rectangles size
    pub fn scale(&mut self, size: Size2D<f32, ObjectSpace>) {
        self.size = size;
    }
}

impl From<Rect<f32, WorldSpace>> for Rectangle {
    fn from(rect: Rect<f32, WorldSpace>) -> Self {
        Self {
            origin: rect.origin.to_3d(),
            size: rect.size.cast_unit(),
            rotation: Rotation3D::identity(),
            color: Rgba::WHITE,
        }
    }
}

impl Drawable for Rectangle {
    type Pipeline = TrianglePipe;

    fn breakdown(&self) -> Breakdown<TriangleVertex> {
        let mut tl: Point3D<f32, ObjectSpace> = Point3D::new(
            -self.size.width / 2.0,
            self.size.height / 2.0,
            self.origin.z
        );
        let mut tr: Point3D<f32, ObjectSpace> = Point3D::new(
            self.size.width / 2.0,
            self.size.height / 2.0,
            self.origin.z
        );
        let mut bl: Point3D<f32, ObjectSpace> = Point3D::new(
            -self.size.width / 2.0,
            -self.size.height / 2.0,
            self.origin.z
        );
        let mut br: Point3D<f32, ObjectSpace> = Point3D::new(
            self.size.width / 2.0,
            -self.size.height / 2.0,
            self.origin.z
        );
        // Rotate each of the points (this must be done in object space)
        for vert in [&mut tl, &mut tr, &mut bl, &mut br] {
            *vert = self.rotation.transform_point3d(*vert);
            vert.x = vert.x + self.origin.x;
            vert.y = vert.y + self.origin.y;
            vert.z = self.origin.z;
        }
        let color = (self.color.r, self.color.g, self.color.b, self.color.a);
        let vertices = vec![
            TriangleVertex::new_from_tuple(tl.to_tuple(), color),
            TriangleVertex::new_from_tuple(tr.to_tuple(), color),
            TriangleVertex::new_from_tuple(bl.to_tuple(), color),
            TriangleVertex::new_from_tuple(br.to_tuple(), color),
        ];

        Breakdown {
            vertices,
            indicies: vec![0, 1, 3, 0, 3, 2],
            texture: None
        }
    }
}

/// A triangle represented by three points and an origin
#[derive(Debug, Clone)]
pub struct Triangle {
    /// First point
    pub point_a: Point3D<f32, ObjectSpace>,
    /// Second point
    pub point_b: Point3D<f32, ObjectSpace>,
    /// Third point
    pub point_c: Point3D<f32, ObjectSpace>,
    /// Rotation
    pub rotation: Rotation3D<f32, ObjectSpace, ObjectSpace>,
    /// The location of the triangle in the world
    pub origin: Point3D<f32, WorldSpace>,
    // The color of the triangle
    pub color: Rgba,
}

impl Triangle {
    /// Create a new triangle
    pub fn new(point_a: impl Into<Point3D<f32, ObjectSpace>>,
        point_b: impl Into<Point3D<f32, ObjectSpace>>,
        point_c: impl Into<Point3D<f32, ObjectSpace>>,
        origin: impl Into<Point3D<f32, WorldSpace>>,
        color: impl Into<Rgba>,
    ) -> Self {
        Self {
            point_a: point_a.into(),
            point_b: point_b.into(),
            point_c: point_c.into(),
            rotation: Rotation3D::identity(),
            origin: origin.into(),
            color: color.into()
        }
    }

    /// Rotate the triangle
    pub fn rotate(&mut self, rotation: Rotation3D<f32, ObjectSpace, ObjectSpace>) {
        self.rotation = rotation;
    }

    /// Translate the triangle
    pub fn translate(&mut self, translation: Translation3D<f32, WorldSpace, WorldSpace>) {
        self.origin = translation.transform_point3d(&self.origin);
    }

    /// Scale the triangle
    pub fn scale(&mut self, x: f32, y: f32, z: f32) {
        let transform = Transform3D::scale(x, y, z);
        self.point_a = transform.transform_point3d(self.point_a).unwrap();
        self.point_b = transform.transform_point3d(self.point_b).unwrap();
        self.point_c = transform.transform_point3d(self.point_c).unwrap();
    }
}

impl Drawable for Triangle {
    type Pipeline = TrianglePipe;

    fn breakdown(&self) -> Breakdown<TriangleVertex> {
        let mut p1 = self.point_a;
        let mut p2 = self.point_b;
        let mut p3 = self.point_c;
        for vert in [&mut p1, &mut p2, &mut p3] {
            *vert = self.rotation.transform_point3d(*vert);
            vert.x = vert.x + self.origin.x;
            vert.y = vert.y + self.origin.y;
            vert.z = self.origin.z;
        }
        let color = (self.color.r, self.color.g, self.color.b, self.color.a);
        let vertices = vec![
            TriangleVertex::new_from_tuple(p1.to_tuple(), color),
            TriangleVertex::new_from_tuple(p2.to_tuple(), color),
            TriangleVertex::new_from_tuple(p3.to_tuple(), color),
        ];

        Breakdown {
            vertices,
            indicies: vec![0, 1, 2],
            texture: None,
        }
    }
}