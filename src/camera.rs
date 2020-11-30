pub use crate::ray::Ray;
pub use vek::{Mat4, Transform, Vec2, Vec3};

/// A perspective camera
#[derive(Clone, Copy, Debug)]
pub struct Camera {
    /// Camera to world transform
    pub transform: Transform<f32, f32, f32>,

    /// Horizontal field of view in degrees
    pub fov: f32,

    /// The ratio of width to height
    pub aspect_ratio: f32,

    /// Near clipping planes in world-space units
    pub near_clip: f32,

    /// Far clipping planes in world-space units
    pub far_clip: f32,
}

impl Camera {
    /// Construct a new camera
    pub fn new(
        transform: Transform<f32, f32, f32>,
        fov: f32,
        aspect_ratio: f32,
        near_clip: f32,
        far_clip: f32,
    ) -> Self {
        Camera {
            transform,
            fov,
            aspect_ratio,
            near_clip,
            far_clip,
        }
    }

    /// Generate ray from a clip space position.
    /// Clip space is a 2 unit wide cube, centered at (0,0,0), and with corners
    /// that range from (-1,-1,-1) to (1,1,1).
    pub fn generate_ray(&self, clip_position: Vec2<f32>) -> Ray {
        let direction_x = 1.0 / (self.fov / 2.0).tan();
        let direction_y = clip_position.x * self.aspect_ratio;
        let direction_z = clip_position.y;
        let direction = Vec3::new(direction_x, direction_y, direction_z);
        Ray::new(
            self.transform.position,
            (Mat4::from(self.transform).mul_direction(direction)).normalized(),
            self.near_clip * direction.magnitude() / direction_x,
            self.far_clip * direction.magnitude() / direction_x,
        )
    }
}
