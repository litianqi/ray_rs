pub use crate::drawable::{Drawable, Intersection};
pub use crate::ray::Ray;
use roots::{find_roots_quadratic, Roots};
pub use vek::{Sphere, Vec3};

impl Drawable for Sphere<f32, f32> {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        // Vector Algebra:
        // v ⋅ v = |v|^2
        // a ⋅ (b + c) = a ⋅ b + a ⋅ c
        // https://en.wikipedia.org/wiki/Dot_product
        //
        // Quadratic Formula:
        // a * x^2 + b * x + c = 0
        // discriminant = b^2 - 4 * a * c
        // x = (-b +/- discriminant.sqrt()) / (2 * a)
        // * when discriminant is positive, we get two real solutions
        // * when it is zero we get just one real solution
        // * when it is negative we get no real solutions
        // https://www.mathsisfun.com/algebra/quadratic-equation.html
        //
        // Our Problem:
        // Ray(t) = origin + t * direction
        // (Ray(t) − center) ⋅ (Ray(t) − center) = radius^2
        // =>
        // (origin + t * direction − center) ⋅ (origin + t * direction − center) = radius^2
        // =>
        // t^2 * direction ⋅ direction + 2 * t * direction ⋅ (origin − center) + (origin − center) ⋅ (origin − center) − radius^2 = 0
        // =>
        // t^2 * |direction|^2 + 2 * t * direction ⋅ (origin − center) + |(origin − center)|^2 − radius^2 = 0
        // =>
        // a = |direction|^2
        // b = 2 * direction ⋅ (origin − center )
        // c = |(origin − center)|^2 − radius^2
        //
        // The vectors and radius in that equation are all constant and known.
        // The unknown is t, and the equation is a quadratic, like you probably
        // saw in your high school math class. You can solve for t and there is
        // a square root part that is either positive (meaning two real
        // solutions), negative (meaning no real solutions), or zero (meaning
        // one real solution).

        let oc = ray.origin - self.center;
        let a = ray.direction.magnitude_squared();
        let b = 2.0 * ray.direction.dot(oc);
        let c = oc.magnitude_squared() - self.radius.powi(2);

        let t = match find_roots_quadratic(a, b, c) {
            Roots::One(roots) => {
                if ray.mint < roots[0] && roots[0] < ray.maxt {
                    roots[0]
                } else {
                    return None;
                }
            }
            Roots::Two(roots) => {
                if ray.mint < roots[0] && roots[0] < ray.maxt {
                    roots[0]
                } else if ray.mint < roots[1] && roots[1] < ray.maxt {
                    roots[1]
                } else {
                    return None;
                }
            }
            _ => return None,
        };

        let position = ray.point_at(t);
        let normal = (position - self.center) / self.radius;
        return Some(Intersection {
            t,
            position,
            normal,
        });
    }
}
