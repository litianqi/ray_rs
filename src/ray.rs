pub use vek::Vec3;
// use std::f32;

/// Simple 3D ray segment data structure
#[derive(Clone, Copy, Debug)]
pub struct Ray {
    /// Ray origin
    pub origin: Vec3<f32>,

    /// Ray direction
    pub direction: Vec3<f32>,

    /// Minimum position on the ray segment
    pub mint: f32,

    /// Maximum position on the ray segment
    pub maxt: f32,
}

impl Ray {
    /// Construct a new ray
    pub fn new(origin: Vec3<f32>, direction: Vec3<f32>, mint: f32, maxt: f32) -> Self {
        Ray {
            origin,
            direction,
            mint,
            maxt,
        }
    }

    /// Return the position of a point along the ray
    pub fn point_at(&self, t: f32) -> Vec3<f32> {
        return self.origin + self.direction * t;
    }

    /// Return a ray that points into the opposite direction
    pub fn reverse(&self) -> Self {
        return Ray {
            origin: self.origin,
            direction: self.direction,
            mint: self.mint,
            maxt: self.maxt,
        };
    }
}
