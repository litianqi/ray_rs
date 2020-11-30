pub use crate::ray::Ray;
pub use vek::Vec3;

/// Intersection data structure
pub struct Intersection {
    /// Distance from the ray origin to the intersection point
    pub t: f32,

    /// Position of the surface intersection
    pub position: Vec3<f32>,

    /// Normal of the surface intersection
    pub normal: Vec3<f32>,
}

/// Trait implemented by scene objects that can be drawn
pub trait Drawable {
    /// Intersect a ray against object and return detailed intersection information
    fn intersect(&self, ray: &Ray) -> Option<Intersection>;
}
